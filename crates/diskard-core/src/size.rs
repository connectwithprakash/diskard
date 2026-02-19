use std::path::Path;

/// Format bytes into a human-readable string using binary units (e.g., "1.0 GiB").
pub fn format_bytes(bytes: u64) -> String {
    bytesize::ByteSize(bytes).to_string_as(true)
}

/// Calculate the total size of a directory by walking all files.
pub fn dir_size(path: &Path) -> u64 {
    if !path.exists() {
        return 0;
    }

    if path.is_file() {
        return path.metadata().map(|m| m.len()).unwrap_or(0);
    }

    jwalk::WalkDir::new(path)
        .skip_hidden(false)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.metadata().map(|m| m.len()).unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1024), "1.0 kiB");
        assert_eq!(format_bytes(1_073_741_824), "1.0 GiB");
    }

    #[test]
    fn test_dir_size_nonexistent() {
        assert_eq!(dir_size(Path::new("/nonexistent/path")), 0);
    }
}
