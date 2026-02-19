use std::path::PathBuf;
use std::process::Command;

use log::{info, warn, error};

use crate::utils::paths;

/// Resolves the zeroclaw binary path by searching multiple locations:
/// 1. ~/.zeroclaw/bin/zeroclaw (installed by client during initialization)
/// 2. System PATH (via `which zeroclaw`)
/// Returns None if not found anywhere.
fn resolve_zeroclaw_bin() -> Option<PathBuf> {
    // Prefer the client-installed binary
    if let Ok(bin_path) = paths::zeroclaw_bin_path() {
        if bin_path.exists() {
            info!("[resolve_zeroclaw_bin] Found client-installed binary: {:?}", bin_path);
            return Some(bin_path);
        }
    }

    // Fallback to system PATH
    if let Ok(output) = Command::new("which").arg("zeroclaw").output() {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                let path = PathBuf::from(&path_str);
                info!("[resolve_zeroclaw_bin] Found in system PATH: {:?}", path);
                return Some(path);
            }
        }
    }

    warn!("[resolve_zeroclaw_bin] zeroclaw binary not found in ~/.zeroclaw/bin/ or system PATH");
    None
}

/// Checks if the ZeroClaw server is running by querying its status.
#[tauri::command]
pub async fn gateway_status() -> Result<bool, String> {
    let bin_path = match resolve_zeroclaw_bin() {
        Some(p) => p,
        None => {
            info!("[gateway_status] zeroclaw binary not found, returning false");
            return Ok(false);
        }
    };

    info!("[gateway_status] Executing: {:?} service status", bin_path);

    let output = Command::new(&bin_path)
        .args(["service", "status"])
        .output()
        .map_err(|e| {
            error!("[gateway_status] Failed to execute command: {}", e);
            format!("Failed to check server status: {}", e)
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    info!("[gateway_status] exit code: {:?}", output.status.code());
    info!("[gateway_status] stdout: {}", stdout);
    info!("[gateway_status] stderr: {}", stderr);
    info!("[gateway_status] success: {}", output.status.success());

    Ok(output.status.success())
}

/// Attempts to start the ZeroClaw server in background mode.
/// Returns Ok with a message on success, or Ok with warning if binary not found.
#[tauri::command]
pub async fn start_gateway() -> Result<String, String> {
    let bin_path = match resolve_zeroclaw_bin() {
        Some(p) => p,
        None => {
            warn!("[start_gateway] ZeroClaw binary not found, skipping startup");
            return Ok("ZeroClaw binary not found, skipping startup".to_string());
        }
    };

    info!("[start_gateway] Starting server with {:?}", bin_path);

    // Ensure service is installed (idempotent), then start
    let install_output = Command::new(&bin_path)
        .args(["service", "install"])
        .output()
        .map_err(|e| {
            error!("[start_gateway] Failed to install service: {}", e);
            format!("Failed to install service: {}", e)
        })?;

    info!("[start_gateway] service install exit={:?} stdout={} stderr={}",
        install_output.status.code(),
        String::from_utf8_lossy(&install_output.stdout),
        String::from_utf8_lossy(&install_output.stderr));

    Command::new(&bin_path)
        .args(["service", "start"])
        .spawn()
        .map_err(|e| {
            error!("[start_gateway] Failed to start server: {}", e);
            format!("Failed to start server: {}", e)
        })?;

    info!("[start_gateway] ZeroClaw server started successfully");
    Ok("ZeroClaw server started successfully".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_zeroclaw_bin_returns_option() {
        let result = resolve_zeroclaw_bin();
        if let Some(path) = &result {
            assert!(!path.to_string_lossy().is_empty());
        }
    }

    #[test]
    fn test_zeroclaw_binary_fallback_path_construction() {
        let zeroclaw_dir = paths::zeroclaw_dir().unwrap();
        let bin_path = zeroclaw_dir.join("bin").join("zeroclaw");
        assert!(bin_path.to_string_lossy().contains(".zeroclaw"));
        assert!(bin_path.to_string_lossy().ends_with("zeroclaw"));
    }
}
