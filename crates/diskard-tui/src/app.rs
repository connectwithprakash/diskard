use diskard_core::finding::Finding;

/// Application state for the TUI.
pub struct App {
    pub findings: Vec<FindingItem>,
    pub selected: usize,
    pub should_quit: bool,
    pub show_help: bool,
    pub mode: AppMode,
    pub status_message: Option<String>,
}

pub struct FindingItem {
    pub finding: Finding,
    pub checked: bool,
}

#[derive(PartialEq)]
pub enum AppMode {
    Browse,
    Confirm,
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
}
