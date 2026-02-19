use std::fs;
use std::path::Path;

/// Installs a binary file from `src` to `dst`.
/// - Creates parent directories if needed
/// - Only copies if `dst` does not already exist (idempotent)
/// - On Unix, sets executable permission (chmod +x)
/// Returns true if the binary was installed, false if it already existed.
pub fn install_binary(src: &Path, dst: &Path) -> Result<bool, String> {
    if dst.exists() {
        return Ok(false);
    }

    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create bin dir {}: {}", parent.display(), e))?;
    }

    fs::copy(src, dst).map_err(|e| {
        format!(
            "Failed to install binary {} -> {}: {}",
            src.display(),
            dst.display(),
            e
        )
    })?;

    // Set executable permission on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(0o755);
        fs::set_permissions(dst, perms).map_err(|e| {
            format!("Failed to set executable permission on {}: {}", dst.display(), e)
        })?;
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_install_binary_copies_and_sets_executable() {
        let tmp = TempDir::new().unwrap();
        let src = tmp.path().join("my_bin");
        let dst = tmp.path().join("installed/my_bin");

        fs::write(&src, "#!/bin/sh\necho hello").unwrap();

        let installed = install_binary(&src, &dst).unwrap();
        assert!(installed);
        assert!(dst.exists());

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = fs::metadata(&dst).unwrap().permissions();
            assert_eq!(perms.mode() & 0o755, 0o755);
        }
    }

    #[test]
    fn test_install_binary_is_idempotent() {
        let tmp = TempDir::new().unwrap();
        let src = tmp.path().join("my_bin");
        let dst = tmp.path().join("my_bin_installed");

        fs::write(&src, "binary v1").unwrap();

        let first = install_binary(&src, &dst).unwrap();
        assert!(first);

        fs::write(&src, "binary v2").unwrap();

        let second = install_binary(&src, &dst).unwrap();
        assert!(!second);

        assert_eq!(fs::read_to_string(&dst).unwrap(), "binary v1");
    }
}
