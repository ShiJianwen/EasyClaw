use std::fs;
use std::path::Path;
use std::process::Command;
use log::{info, warn, error};
use tauri::Manager;

use crate::utils::fs::install_binary;
use crate::utils::paths;

/// Checks whether ZeroClaw has been initialized by verifying
/// the existence of ~/.zeroclaw/config.toml.
#[tauri::command]
pub async fn check_initialized() -> Result<bool, String> {
    let config_path = paths::config_file_path()?;
    Ok(config_path.exists())
}

/// Runs `zeroclaw onboard` to generate config, workspace structure, and all template files.
/// Skipped if config.toml already exists (idempotent).
fn run_zeroclaw_onboard(bin_path: &Path, zeroclaw_dir: &Path) -> Result<(), String> {
    let config_path = zeroclaw_dir.join("config.toml");
    if config_path.exists() {
        info!("[run_zeroclaw_onboard] config.toml already exists, skipping onboard");
        return Ok(());
    }

    info!("[run_zeroclaw_onboard] Running zeroclaw onboard to generate config and workspace");

    let output = Command::new(bin_path)
        .args(["onboard", "--provider", "bailian", "--model", "qwen3-max-2026-01-23", "--memory", "sqlite"])
        .output()
        .map_err(|e| {
            error!("[run_zeroclaw_onboard] Failed to execute onboard: {}", e);
            format!("Failed to run zeroclaw onboard: {}", e)
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    info!("[run_zeroclaw_onboard] exit code: {:?}", output.status.code());
    info!("[run_zeroclaw_onboard] stdout: {}", stdout);
    if !stderr.is_empty() {
        warn!("[run_zeroclaw_onboard] stderr: {}", stderr);
    }

    if !output.status.success() {
        return Err(format!("zeroclaw onboard failed (exit {}): {}",
            output.status.code().unwrap_or(-1), stderr));
    }

    // Fix file permissions: chmod 600 config.toml
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if config_path.exists() {
            let perms = fs::Permissions::from_mode(0o600);
            fs::set_permissions(&config_path, perms).map_err(|e| {
                format!("Failed to set config permissions: {}", e)
            })?;
            info!("[run_zeroclaw_onboard] Set config.toml permissions to 600");
        }
    }

    Ok(())
}

/// Performs the full ZeroClaw initialization:
/// 1. Installs zeroclaw binary to ~/.zeroclaw/bin/
/// 2. Runs `zeroclaw onboard` to generate config.toml, workspace, and all template files
///
/// This operation is idempotent - safe to call multiple times.
#[tauri::command]
pub async fn initialize_zeroclaw(app: tauri::AppHandle) -> Result<String, String> {
    let zeroclaw_dir = paths::zeroclaw_dir()?;

    // Get resource directory from Tauri app handle
    let resource_dir = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Failed to get resource directory: {}", e))?;

    // Step 1: Install zeroclaw binary to ~/.zeroclaw/bin/
    let bin_src = resource_dir.join("resources").join("bin").join("zeroclaw");
    let bin_dst = paths::zeroclaw_bin_path()?;
    if bin_src.exists() {
        install_binary(&bin_src, &bin_dst)?;
    }

    // Step 2: Run zeroclaw onboard (generates everything: config, workspace, templates, skills)
    if bin_dst.exists() {
        run_zeroclaw_onboard(&bin_dst, &zeroclaw_dir)?;
    }

    Ok("ZeroClaw initialization completed successfully".to_string())
}

/// Standalone initialization logic that can be tested without Tauri AppHandle.
pub fn initialize_from_resource_dir(
    resource_dir: &Path,
    zeroclaw_dir: &Path,
) -> Result<String, String> {
    // Install zeroclaw binary
    let bin_src = resource_dir.join("bin").join("zeroclaw");
    let bin_dst = zeroclaw_dir.join("bin").join("zeroclaw");
    if bin_src.exists() {
        install_binary(&bin_src, &bin_dst)?;
    }

    // Run zeroclaw onboard if binary exists
    if bin_dst.exists() {
        run_zeroclaw_onboard(&bin_dst, zeroclaw_dir)?;
    }

    Ok("ZeroClaw initialization completed successfully".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Creates a mock zeroclaw binary that simulates `onboard`:
    /// generates config.toml and workspace structure.
    fn create_mock_binary(dir: &Path, zeroclaw_dir: &Path) {
        fs::create_dir_all(dir.join("bin")).unwrap();
        let script = format!(
            r##"#!/bin/sh
mkdir -p "{zd}"
mkdir -p "{zd}/workspace/sessions"
mkdir -p "{zd}/workspace/memory"
mkdir -p "{zd}/workspace/skills"
cat > "{zd}/config.toml" << 'EOF'
default_provider = "bailian"
default_model = "qwen3-max-2026-01-23"
[memory]
backend = "sqlite"
auto_save = true
EOF
echo "# Memory" > "{zd}/workspace/MEMORY.md"
echo "# User" > "{zd}/workspace/USER.md"
echo "# Soul" > "{zd}/workspace/SOUL.md"
"##,
            zd = zeroclaw_dir.display()
        );
        fs::write(dir.join("bin/zeroclaw"), script).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(
                dir.join("bin/zeroclaw"),
                fs::Permissions::from_mode(0o755),
            )
            .unwrap();
        }
    }

    #[test]
    fn test_initialize_creates_all_expected_files() {
        let resource_dir = TempDir::new().unwrap();
        let target_dir = TempDir::new().unwrap();
        let zeroclaw_dir = target_dir.path().join(".zeroclaw");

        create_mock_binary(resource_dir.path(), &zeroclaw_dir);

        let result = initialize_from_resource_dir(resource_dir.path(), &zeroclaw_dir);
        assert!(result.is_ok());

        // Verify config created by onboard
        assert!(zeroclaw_dir.join("config.toml").exists());
        let config = fs::read_to_string(zeroclaw_dir.join("config.toml")).unwrap();
        assert!(config.contains("auto_save"));

        // Verify workspace files created by onboard
        assert!(zeroclaw_dir.join("workspace/MEMORY.md").exists());
        assert!(zeroclaw_dir.join("workspace/USER.md").exists());
        assert!(zeroclaw_dir.join("workspace/SOUL.md").exists());
        assert!(zeroclaw_dir.join("workspace/skills").is_dir());

        // Verify binary installed with executable permission
        let bin_path = zeroclaw_dir.join("bin/zeroclaw");
        assert!(bin_path.exists());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = fs::metadata(&bin_path).unwrap().permissions();
            assert_eq!(perms.mode() & 0o755, 0o755);
        }
    }

    #[test]
    fn test_initialize_is_idempotent() {
        let resource_dir = TempDir::new().unwrap();
        let target_dir = TempDir::new().unwrap();
        let zeroclaw_dir = target_dir.path().join(".zeroclaw");

        create_mock_binary(resource_dir.path(), &zeroclaw_dir);

        // First init
        initialize_from_resource_dir(resource_dir.path(), &zeroclaw_dir).unwrap();

        // Modify a workspace file
        let memory_path = zeroclaw_dir.join("workspace/MEMORY.md");
        fs::write(&memory_path, "# Custom Memory").unwrap();

        // Second init — onboard skipped because config.toml exists
        initialize_from_resource_dir(resource_dir.path(), &zeroclaw_dir).unwrap();

        // User's changes preserved
        assert_eq!(fs::read_to_string(&memory_path).unwrap(), "# Custom Memory");
    }

    #[test]
    fn test_initialize_handles_missing_binary_gracefully() {
        let resource_dir = TempDir::new().unwrap();
        let target_dir = TempDir::new().unwrap();
        let zeroclaw_dir = target_dir.path().join(".zeroclaw");

        // No binary in resources
        let result = initialize_from_resource_dir(resource_dir.path(), &zeroclaw_dir);
        assert!(result.is_ok());
        assert!(!zeroclaw_dir.join("config.toml").exists());
    }
}
