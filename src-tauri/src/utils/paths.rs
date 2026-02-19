use std::path::PathBuf;

/// Returns the ZeroClaw root directory: ~/.zeroclaw
pub fn zeroclaw_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "Cannot determine home directory".to_string())?;
    Ok(home.join(".zeroclaw"))
}

/// Returns the ZeroClaw workspace directory: ~/.zeroclaw/workspace
pub fn workspace_dir() -> Result<PathBuf, String> {
    Ok(zeroclaw_dir()?.join("workspace"))
}

/// Returns the ZeroClaw bin directory: ~/.zeroclaw/bin
pub fn bin_dir() -> Result<PathBuf, String> {
    Ok(zeroclaw_dir()?.join("bin"))
}

/// Returns the ZeroClaw binary path: ~/.zeroclaw/bin/zeroclaw
pub fn zeroclaw_bin_path() -> Result<PathBuf, String> {
    Ok(bin_dir()?.join("zeroclaw"))
}

/// Returns the ZeroClaw config file path: ~/.zeroclaw/config.toml
pub fn config_file_path() -> Result<PathBuf, String> {
    Ok(zeroclaw_dir()?.join("config.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeroclaw_dir_is_under_home() {
        let dir = zeroclaw_dir().unwrap();
        let home = dirs::home_dir().unwrap();
        assert!(dir.starts_with(&home));
        assert!(dir.ends_with(".zeroclaw"));
    }

    #[test]
    fn test_workspace_dir_is_under_zeroclaw_dir() {
        let ws = workspace_dir().unwrap();
        let zc = zeroclaw_dir().unwrap();
        assert!(ws.starts_with(&zc));
        assert!(ws.ends_with("workspace"));
    }

    #[test]
    fn test_config_file_path_ends_with_toml() {
        let cfg = config_file_path().unwrap();
        assert!(cfg.to_string_lossy().ends_with("config.toml"));
    }

    #[test]
    fn test_bin_dir_is_under_zeroclaw_dir() {
        let bd = bin_dir().unwrap();
        let zc = zeroclaw_dir().unwrap();
        assert!(bd.starts_with(&zc));
        assert!(bd.ends_with("bin"));
    }

    #[test]
    fn test_zeroclaw_bin_path_ends_with_zeroclaw() {
        let bp = zeroclaw_bin_path().unwrap();
        assert!(bp.to_string_lossy().contains(".zeroclaw/bin/zeroclaw"));
    }
}
