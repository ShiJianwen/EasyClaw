use std::fs;
use std::path::Path;

/// Recursively copies the contents of `src` directory into `dst` directory.
/// Creates `dst` if it doesn't exist. Overwrites existing files.
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), String> {
    if !src.is_dir() {
        return Err(format!("Source is not a directory: {}", src.display()));
    }

    fs::create_dir_all(dst).map_err(|e| format!("Failed to create dir {}: {}", dst.display(), e))?;

    let entries = fs::read_dir(src)
        .map_err(|e| format!("Failed to read dir {}: {}", src.display(), e))?;

    for entry in entries {
        let entry =
            entry.map_err(|e| format!("Failed to read entry in {}: {}", src.display(), e))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(|e| {
                format!(
                    "Failed to copy {} -> {}: {}",
                    src_path.display(),
                    dst_path.display(),
                    e
                )
            })?;
        }
    }

    Ok(())
}

/// Copies a file from `src` to `dst` only if `dst` does not already exist.
/// Creates parent directories if needed.
pub fn copy_file_if_not_exists(src: &Path, dst: &Path) -> Result<bool, String> {
    if dst.exists() {
        return Ok(false);
    }

    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create parent dir {}: {}", parent.display(), e))?;
    }

    fs::copy(src, dst).map_err(|e| {
        format!(
            "Failed to copy {} -> {}: {}",
            src.display(),
            dst.display(),
            e
        )
    })?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_copy_dir_recursive_creates_dst_and_copies_files() {
        let src_dir = TempDir::new().unwrap();
        let dst_dir = TempDir::new().unwrap();
        let dst_path = dst_dir.path().join("output");

        // Create source structure: file.txt, sub/nested.txt
        fs::write(src_dir.path().join("file.txt"), "hello").unwrap();
        fs::create_dir_all(src_dir.path().join("sub")).unwrap();
        fs::write(src_dir.path().join("sub/nested.txt"), "world").unwrap();

        copy_dir_recursive(src_dir.path(), &dst_path).unwrap();

        assert!(dst_path.join("file.txt").exists());
        assert_eq!(fs::read_to_string(dst_path.join("file.txt")).unwrap(), "hello");
        assert!(dst_path.join("sub/nested.txt").exists());
        assert_eq!(
            fs::read_to_string(dst_path.join("sub/nested.txt")).unwrap(),
            "world"
        );
    }

    #[test]
    fn test_copy_dir_recursive_fails_on_non_dir_source() {
        let tmp = TempDir::new().unwrap();
        let file_path = tmp.path().join("file.txt");
        fs::write(&file_path, "data").unwrap();

        let result = copy_dir_recursive(&file_path, &tmp.path().join("out"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Source is not a directory"));
    }

    #[test]
    fn test_copy_file_if_not_exists_copies_when_missing() {
        let tmp = TempDir::new().unwrap();
        let src = tmp.path().join("src.txt");
        let dst = tmp.path().join("subdir/dst.txt");

        fs::write(&src, "content").unwrap();

        let copied = copy_file_if_not_exists(&src, &dst).unwrap();
        assert!(copied);
        assert_eq!(fs::read_to_string(&dst).unwrap(), "content");
    }

    #[test]
    fn test_copy_file_if_not_exists_skips_when_present() {
        let tmp = TempDir::new().unwrap();
        let src = tmp.path().join("src.txt");
        let dst = tmp.path().join("dst.txt");

        fs::write(&src, "new content").unwrap();
        fs::write(&dst, "old content").unwrap();

        let copied = copy_file_if_not_exists(&src, &dst).unwrap();
        assert!(!copied);
        assert_eq!(fs::read_to_string(&dst).unwrap(), "old content");
    }
}
