use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;
use crate::size::dir_size;
use std::collections::HashMap;

/// VS Code extensions — detects duplicate/old versions.
pub struct VSCodeExtensions;

impl Recognizer for VSCodeExtensions {
    fn name(&self) -> &'static str {
        "VS Code extensions"
    }

    fn id(&self) -> &'static str {
        "vscode-extensions"
    }

    fn category(&self) -> Category {
        Category::VSCode
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = dirs::home_dir() else {
            return Ok(vec![]);
        };
        let extensions_dir = home.join(".vscode/extensions");
        if !extensions_dir.exists() {
            return Ok(vec![]);
        }

        // Group extensions by name (without version) to detect duplicates
        let mut extensions: HashMap<String, Vec<std::path::PathBuf>> = HashMap::new();

        let entries = std::fs::read_dir(&extensions_dir)
            .map_err(|e| crate::error::Error::io(&extensions_dir, e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            // Extension dirs are like "publisher.name-1.2.3"
            // Strip the version suffix to group by extension identity
            if let Some(base) = strip_version_suffix(&name) {
                extensions.entry(base).or_default().push(path);
            }
        }

        let mut findings = Vec::new();

        for (ext_name, mut versions) in extensions {
            if versions.len() <= 1 {
                continue;
            }

            // Sort by name (version order) and mark all but the last as old
            versions.sort();
            let old_versions = &versions[..versions.len() - 1];

            for old in old_versions {
                let size = dir_size(old);
                if size > 0 {
                    findings.push(Finding {
                        path: old.clone(),
                        category: Category::VSCode,
                        risk: RiskLevel::Moderate,
                        size_bytes: size,
                        description: format!("Old version of VS Code extension {ext_name}"),
                        last_modified: None,
                    });
                }
            }
        }

        Ok(findings)
    }
}

/// Strip the version suffix from a VS Code extension directory name.
/// "publisher.name-1.2.3" -> "publisher.name"
fn strip_version_suffix(name: &str) -> Option<String> {
    // Find the last '-' followed by a version-like pattern
    if let Some(idx) = name.rfind('-') {
        let version_part = &name[idx + 1..];
        // Check if it looks like a version (starts with a digit)
        if version_part
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit())
        {
            return Some(name[..idx].to_string());
        }
    }
    Some(name.to_string())
}
