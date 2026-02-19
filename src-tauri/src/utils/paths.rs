use std::path::PathBuf;

/// Returns the OpenClaw root directory: ~/.openclaw
pub fn openclaw_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "Cannot determine home directory".to_string())?;
    Ok(home.join(".openclaw"))
}

/// Returns the OpenClaw workspace directory: ~/.openclaw/workspace
pub fn workspace_dir() -> Result<PathBuf, String> {
    Ok(openclaw_dir()?.join("workspace"))
}

/// Returns the OpenClaw config file path: ~/.openclaw/openclaw.json
pub fn config_file_path() -> Result<PathBuf, String> {
    Ok(openclaw_dir()?.join("openclaw.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openclaw_dir_is_under_home() {
        let dir = openclaw_dir().unwrap();
        let home = dirs::home_dir().unwrap();
        assert!(dir.starts_with(&home));
        assert!(dir.ends_with(".openclaw"));
    }

    #[test]
    fn test_workspace_dir_is_under_openclaw_dir() {
        let ws = workspace_dir().unwrap();
        let oc = openclaw_dir().unwrap();
        assert!(ws.starts_with(&oc));
        assert!(ws.ends_with("workspace"));
    }

    #[test]
    fn test_config_file_path_ends_with_json() {
        let cfg = config_file_path().unwrap();
        assert!(cfg.to_string_lossy().ends_with("openclaw.json"));
    }
}
