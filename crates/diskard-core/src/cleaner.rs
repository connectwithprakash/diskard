use std::path::Path;

use crate::error::{Error, Result};
use crate::finding::Finding;

/// How to delete files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteMode {
    /// Move to system trash (default, recoverable).
    Trash,
    /// Permanently delete (irreversible).
    Permanent,
    /// Only show what would be deleted.
    DryRun,
}

/// Results from a clean operation.
pub struct CleanResult {
    pub deleted_count: usize,
    pub freed_bytes: u64,
    pub errors: Vec<(String, String)>,
}

/// Delete the given findings using the specified mode.
pub fn clean(findings: &[Finding], mode: DeleteMode) -> Result<CleanResult> {
    let mut deleted_count = 0;
    let mut freed_bytes = 0;
    let mut errors: Vec<(String, String)> = Vec::new();

    for finding in findings {
        if mode == DeleteMode::DryRun {
            deleted_count += 1;
            freed_bytes += finding.size_bytes;
            continue;
        }

        match delete_path(&finding.path, mode) {
            Ok(()) => {
                deleted_count += 1;
                freed_bytes += finding.size_bytes;
            }
            Err(e) => {
                errors.push((finding.path.display().to_string(), e.to_string()));
            }
        }
    }

    Ok(CleanResult {
        deleted_count,
        freed_bytes,
        errors,
    })
}

/// Delete a single path using the specified mode.
pub fn delete_path(path: &Path, mode: DeleteMode) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    match mode {
        DeleteMode::Trash => {
            trash::delete(path).map_err(|e| Error::Trash(e.to_string()))?;
        }
        DeleteMode::Permanent => {
            if path.is_dir() {
                std::fs::remove_dir_all(path).map_err(|e| Error::io(path, e))?;
            } else {
                std::fs::remove_file(path).map_err(|e| Error::io(path, e))?;
            }
        }
        DeleteMode::DryRun => {
            // No-op
        }
    }

    Ok(())
}
