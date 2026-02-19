use std::process::Command;

use crate::utils::paths;

/// Checks if the Gateway process is running by looking for the openclaw binary
/// and attempting to query its status.
#[tauri::command]
pub async fn gateway_status() -> Result<bool, String> {
    let openclaw_dir = paths::openclaw_dir()?;
    let bin_path = openclaw_dir.join("bin").join("openclaw");

    if !bin_path.exists() {
        return Ok(false);
    }

    let output = Command::new(&bin_path)
        .args(["gateway", "status"])
        .output()
        .map_err(|e| format!("Failed to check gateway status: {}", e))?;

    Ok(output.status.success())
}

/// Attempts to start the Gateway service in background mode.
/// Returns Ok with a message on success, or Ok with warning if binary not found.
#[tauri::command]
pub async fn start_gateway() -> Result<String, String> {
    let openclaw_dir = paths::openclaw_dir()?;
    let bin_path = openclaw_dir.join("bin").join("openclaw");

    if !bin_path.exists() {
        return Ok("Gateway binary not found, skipping startup".to_string());
    }

    Command::new(&bin_path)
        .args(["gateway", "--background"])
        .spawn()
        .map_err(|e| format!("Failed to start gateway: {}", e))?;

    Ok("Gateway started successfully".to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_gateway_binary_path_construction() {
        use crate::utils::paths;
        let openclaw_dir = paths::openclaw_dir().unwrap();
        let bin_path = openclaw_dir.join("bin").join("openclaw");
        // Should construct a valid path under ~/.openclaw/bin/openclaw
        assert!(bin_path.to_string_lossy().contains(".openclaw"));
        assert!(bin_path.to_string_lossy().ends_with("openclaw"));
    }
}
