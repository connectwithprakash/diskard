use crate::error::Result;
use crate::finding::{Category, Finding, RiskLevel};
use crate::recognizer::Recognizer;
use crate::size::dir_size;

/// Claude Code session and debug data.
pub struct ClaudeData;

impl Recognizer for ClaudeData {
    fn name(&self) -> &'static str {
        "Claude Code data"
    }

    fn id(&self) -> &'static str {
        "claude-data"
    }

    fn category(&self) -> Category {
        Category::Claude
    }

    fn scan(&self) -> Result<Vec<Finding>> {
        let Some(home) = dirs::home_dir() else {
            return Ok(vec![]);
        };

        let mut findings = Vec::new();

        // Debug logs
        let debug_path = home.join(".claude/debug");
        if debug_path.exists() {
            let size = dir_size(&debug_path);
            if size > 0 {
                findings.push(Finding {
                    path: debug_path,
                    category: Category::Claude,
                    risk: RiskLevel::Safe,
                    size_bytes: size,
                    description: "Claude Code debug logs".into(),
                    last_modified: None,
                });
            }
        }

        // Project session transcripts
        let projects_path = home.join(".claude/projects");
        if projects_path.exists() {
            let size = dir_size(&projects_path);
            if size > 0 {
                findings.push(Finding {
                    path: projects_path,
                    category: Category::Claude,
                    risk: RiskLevel::Moderate,
                    size_bytes: size,
                    description: "Claude Code session transcripts and project data".into(),
                    last_modified: None,
                });
            }
        }

        Ok(findings)
    }
}
