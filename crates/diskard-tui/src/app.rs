use std::path::{Path, PathBuf};

use diskard_core::finding::Finding;
use diskard_core::size::dir_size;

/// Application state for the TUI.
pub struct App {
    pub findings: Vec<FindingItem>,
    pub selected: usize,
    pub should_quit: bool,
    pub show_help: bool,
    pub mode: AppMode,
    pub status_message: Option<String>,
    pub drill_down: Option<DrillDownState>,
}

pub struct FindingItem {
    pub finding: Finding,
    pub checked: bool,
}

#[derive(PartialEq)]
pub enum AppMode {
    Browse,
    Confirm,
    DrillDown,
}

/// A single entry (file or directory) inside a drill-down listing.
pub struct DrillDownEntry {
    pub name: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub is_dir: bool,
}

/// State for the drill-down inspector.
pub struct DrillDownState {
    /// Stack of directories visited (first = root finding, last = current).
    pub stack: Vec<PathBuf>,
    /// Entries in the current directory.
    pub entries: Vec<DrillDownEntry>,
    /// Currently highlighted entry index.
    pub selected: usize,
}

/// Scan immediate children of `path` and return entries sorted by size descending.
fn scan_directory(path: &Path) -> Option<Vec<DrillDownEntry>> {
    let read_dir = std::fs::read_dir(path).ok()?;
    let mut entries: Vec<DrillDownEntry> = read_dir
        .filter_map(|e| e.ok())
        .map(|entry| {
            let path = entry.path();
            let is_dir = path.is_dir();
            let size_bytes = if is_dir {
                dir_size(&path)
            } else {
                path.metadata().map(|m| m.len()).unwrap_or(0)
            };
            let name = entry.file_name().to_string_lossy().into_owned();
            DrillDownEntry {
                name,
                path,
                size_bytes,
                is_dir,
            }
        })
        .collect();
    entries.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
    Some(entries)
}

impl DrillDownState {
    /// Create a new drill-down state rooted at `path`.
    fn new(path: PathBuf) -> Option<Self> {
        let entries = scan_directory(&path)?;
        Some(Self {
            stack: vec![path],
            entries,
            selected: 0,
        })
    }

    /// Drill into the currently selected entry (must be a directory).
    pub fn drill_into(&mut self) -> bool {
        if let Some(entry) = self.entries.get(self.selected) {
            if entry.is_dir {
                let child_path = entry.path.clone();
                if let Some(entries) = scan_directory(&child_path) {
                    self.stack.push(child_path);
                    self.entries = entries;
                    self.selected = 0;
                    return true;
                }
            }
        }
        false
    }

    /// Go back one level. Returns `false` if already at the root.
    pub fn go_back(&mut self) -> bool {
        if self.stack.len() <= 1 {
            return false;
        }
        // Peek at the parent before popping to avoid inconsistent state on failure.
        let parent = self.stack[self.stack.len() - 2].clone();
        if let Some(entries) = scan_directory(&parent) {
            self.stack.pop();
            self.entries = entries;
            self.selected = 0;
            true
        } else {
            // Rescan failed — stay on current directory.
            false
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.entries.len() {
            self.selected += 1;
        }
    }

    pub fn total_size(&self) -> u64 {
        self.entries.iter().map(|e| e.size_bytes).sum()
    }

    /// Current directory being viewed.
    pub fn current_path(&self) -> &Path {
        self.stack.last().unwrap()
    }
}

impl App {
    pub fn new(findings: Vec<Finding>) -> Self {
        let items = findings
            .into_iter()
            .map(|f| FindingItem {
                finding: f,
                checked: false,
            })
            .collect();

        Self {
            findings: items,
            selected: 0,
            should_quit: false,
            show_help: false,
            mode: AppMode::Browse,
            status_message: None,
            drill_down: None,
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.findings.len() {
            self.selected += 1;
        }
    }

    pub fn toggle_selected(&mut self) {
        if let Some(item) = self.findings.get_mut(self.selected) {
            item.checked = !item.checked;
        }
    }

    pub fn select_all(&mut self) {
        let all_checked = self.findings.iter().all(|f| f.checked);
        for item in &mut self.findings {
            item.checked = !all_checked;
        }
    }

    pub fn checked_count(&self) -> usize {
        self.findings.iter().filter(|f| f.checked).count()
    }

    pub fn checked_size(&self) -> u64 {
        self.findings
            .iter()
            .filter(|f| f.checked)
            .map(|f| f.finding.size_bytes)
            .sum()
    }

    pub fn checked_findings(&self) -> Vec<Finding> {
        self.findings
            .iter()
            .filter(|f| f.checked)
            .map(|f| f.finding.clone())
            .collect()
    }

    pub fn remove_checked(&mut self) {
        self.findings.retain(|f| !f.checked);
        if self.selected >= self.findings.len() && !self.findings.is_empty() {
            self.selected = self.findings.len() - 1;
        }
    }

    /// Enter drill-down mode for the currently selected finding.
    pub fn enter_drill_down(&mut self) {
        if let Some(item) = self.findings.get(self.selected) {
            let path = &item.finding.path;
            if path.is_dir() {
                if let Some(state) = DrillDownState::new(path.clone()) {
                    self.drill_down = Some(state);
                    self.mode = AppMode::DrillDown;
                } else {
                    self.status_message = Some(" Cannot read directory.".into());
                }
            } else {
                self.status_message = Some(" Not a directory.".into());
            }
        }
    }

    /// Exit drill-down mode entirely, returning to Browse.
    pub fn exit_drill_down(&mut self) {
        self.drill_down = None;
        self.mode = AppMode::Browse;
    }
}
