use std::fs;
use std::path::PathBuf;
use tauri::Manager;

use crate::utils::fs::{copy_dir_recursive, copy_file_if_not_exists};
use crate::utils::paths;

/// Checks whether OpenClaw has been initialized by verifying
/// the existence of ~/.openclaw/openclaw.json.
#[tauri::command]
pub async fn check_initialized() -> Result<bool, String> {
    let config_path = paths::config_file_path()?;
    Ok(config_path.exists())
}

/// Performs the full OpenClaw initialization:
/// 1. Creates ~/.openclaw/workspace directory structure
/// 2. Copies openclaw.json to ~/.openclaw/
/// 3. Copies template files (MEMORY.md, USER.md, SOUL.md) to workspace
/// 4. Copies skill packs to workspace/skills
///
/// This operation is idempotent - existing files are not overwritten.
#[tauri::command]
pub async fn initialize_openclaw(app: tauri::AppHandle) -> Result<String, String> {
    let openclaw_dir = paths::openclaw_dir()?;
    let workspace_dir = paths::workspace_dir()?;

    // Step 1: Create directory structure
    fs::create_dir_all(&workspace_dir)
        .map_err(|e| format!("Failed to create workspace directory: {}", e))?;

    // Get resource directory from Tauri app handle
    let resource_dir = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Failed to get resource directory: {}", e))?;

    // Step 2: Copy main config file
    let config_src = resource_dir.join("resources").join("openclaw.json");
    let config_dst = openclaw_dir.join("openclaw.json");
    if config_src.exists() {
        copy_file_if_not_exists(&config_src, &config_dst)?;
    }

    // Step 3: Copy template files to workspace
    let template_files = ["MEMORY.md", "USER.md", "SOUL.md"];
    for file_name in template_files.iter() {
        let src = resource_dir.join("resources").join(file_name);
        let dst = workspace_dir.join(file_name);
        if src.exists() {
            copy_file_if_not_exists(&src, &dst)?;
        }
    }

    // Step 4: Copy skill packs to workspace/skills
    let skills_src = resource_dir.join("resources").join("skills");
    let skills_dst = workspace_dir.join("skills");
    if skills_src.exists() && skills_src.is_dir() {
        copy_dir_recursive(&skills_src, &skills_dst)?;
    }

    Ok("OpenClaw initialization completed successfully".to_string())
}

/// Standalone initialization logic that can be tested without Tauri AppHandle.
/// Takes explicit resource_dir parameter instead of deriving from AppHandle.
pub fn initialize_from_resource_dir(
    resource_dir: &PathBuf,
    openclaw_dir: &PathBuf,
    workspace_dir: &PathBuf,
) -> Result<String, String> {
    // Create directory structure
    fs::create_dir_all(workspace_dir)
        .map_err(|e| format!("Failed to create workspace directory: {}", e))?;

    // Copy main config file
    let config_src = resource_dir.join("openclaw.json");
    let config_dst = openclaw_dir.join("openclaw.json");
    if config_src.exists() {
        copy_file_if_not_exists(&config_src, &config_dst)?;
    }

    // Copy template files
    let template_files = ["MEMORY.md", "USER.md", "SOUL.md"];
    for file_name in template_files.iter() {
        let src = resource_dir.join(file_name);
        let dst = workspace_dir.join(file_name);
        if src.exists() {
            copy_file_if_not_exists(&src, &dst)?;
        }
    }

    // Copy skill packs
    let skills_src = resource_dir.join("skills");
    let skills_dst = workspace_dir.join("skills");
    if skills_src.exists() && skills_src.is_dir() {
        copy_dir_recursive(&skills_src, &skills_dst)?;
    }

    Ok("OpenClaw initialization completed successfully".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_mock_resources(dir: &std::path::Path) {
        fs::write(dir.join("openclaw.json"), r#"{"model":"test"}"#).unwrap();
        fs::write(dir.join("MEMORY.md"), "# Memory").unwrap();
        fs::write(dir.join("USER.md"), "# User").unwrap();
        fs::write(dir.join("SOUL.md"), "# Soul").unwrap();
        fs::create_dir_all(dir.join("skills/dida")).unwrap();
        fs::write(dir.join("skills/dida/README.md"), "# Dida").unwrap();
        fs::create_dir_all(dir.join("skills/obsidian")).unwrap();
        fs::write(dir.join("skills/obsidian/README.md"), "# Obsidian").unwrap();
    }

    #[test]
    fn test_initialize_creates_all_expected_files() {
        let resource_dir = TempDir::new().unwrap();
        let target_dir = TempDir::new().unwrap();

        let openclaw_dir = target_dir.path().join(".openclaw");
        let workspace_dir = openclaw_dir.join("workspace");

        setup_mock_resources(resource_dir.path());

        let result = initialize_from_resource_dir(
            &resource_dir.path().to_path_buf(),
            &openclaw_dir,
            &workspace_dir,
        );

        assert!(result.is_ok());

        // Verify config file
        assert!(openclaw_dir.join("openclaw.json").exists());
        let config_content =
            fs::read_to_string(openclaw_dir.join("openclaw.json")).unwrap();
        assert!(config_content.contains("test"));

        // Verify template files
        assert!(workspace_dir.join("MEMORY.md").exists());
        assert!(workspace_dir.join("USER.md").exists());
        assert!(workspace_dir.join("SOUL.md").exists());

        // Verify skill packs
        assert!(workspace_dir.join("skills/dida/README.md").exists());
        assert!(workspace_dir.join("skills/obsidian/README.md").exists());
    }

    #[test]
    fn test_initialize_is_idempotent() {
        let resource_dir = TempDir::new().unwrap();
        let target_dir = TempDir::new().unwrap();

        let openclaw_dir = target_dir.path().join(".openclaw");
        let workspace_dir = openclaw_dir.join("workspace");

        setup_mock_resources(resource_dir.path());

        // First initialization
        initialize_from_resource_dir(
            &resource_dir.path().to_path_buf(),
            &openclaw_dir,
            &workspace_dir,
        )
        .unwrap();

        // Modify an existing file
        fs::write(workspace_dir.join("MEMORY.md"), "# Custom Memory").unwrap();

        // Second initialization - should not overwrite
        initialize_from_resource_dir(
            &resource_dir.path().to_path_buf(),
            &openclaw_dir,
            &workspace_dir,
        )
        .unwrap();

        // Verify existing file was not overwritten
        let memory_content =
            fs::read_to_string(workspace_dir.join("MEMORY.md")).unwrap();
        assert_eq!(memory_content, "# Custom Memory");
    }

    #[test]
    fn test_initialize_handles_missing_resource_files_gracefully() {
        let resource_dir = TempDir::new().unwrap();
        let target_dir = TempDir::new().unwrap();

        let openclaw_dir = target_dir.path().join(".openclaw");
        let workspace_dir = openclaw_dir.join("workspace");

        // Only create config, no template files or skills
        fs::write(resource_dir.path().join("openclaw.json"), "{}").unwrap();

        let result = initialize_from_resource_dir(
            &resource_dir.path().to_path_buf(),
            &openclaw_dir,
            &workspace_dir,
        );

        assert!(result.is_ok());
        assert!(openclaw_dir.join("openclaw.json").exists());
        // Template files should not exist since they weren't in resources
        assert!(!workspace_dir.join("MEMORY.md").exists());
    }
}
