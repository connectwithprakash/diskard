use diskard_core::size::format_bytes;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;

/// Render the disk summary header.
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let total = app.disk_total;
    let free = app.disk_free;
    let used = total.saturating_sub(free);
    let used_ratio = if total > 0 {
        used as f64 / total as f64
    } else {
        0.0
    };
    let used_pct = (used_ratio * 100.0) as u8;

    let gauge_color = if used_pct > 85 {
        Color::Red
    } else if used_pct >= 70 {
        Color::Yellow
    } else {
        Color::Green
    };

    // Line 1: disk stats
    let line1 = Line::from(vec![
        Span::styled(" Disk: ", Style::default().fg(Color::White)),
        Span::styled(format_bytes(total), Style::default().fg(Color::Cyan)),
        Span::raw(" total | "),
        Span::styled(format_bytes(used), Style::default().fg(gauge_color)),
        Span::raw(" used | "),
        Span::styled(format_bytes(free), Style::default().fg(Color::Green)),
        Span::raw(" free"),
    ]);

    // Line 2: gauge bar
    // Inner width = area.width - 2 (borders). Reserve space for " XX% used" label.
    let inner_width = area.width.saturating_sub(2) as usize;
    let label = format!(" {used_pct}% used");
    let bar_width = inner_width.saturating_sub(label.len() + 1);
    let filled = (used_ratio * bar_width as f64) as usize;
    let empty = bar_width.saturating_sub(filled);

    let line2 = Line::from(vec![
        Span::raw(" "),
        Span::styled("█".repeat(filled), Style::default().fg(gauge_color)),
        Span::styled("░".repeat(empty), Style::default().fg(Color::DarkGray)),
        Span::styled(label, Style::default().fg(gauge_color)),
    ]);

    // Line 3: reclaimable / selected
    let reclaimable = app.total_reclaimable();
    let selected = app.checked_size();
    let line3 = Line::from(vec![
        Span::styled(" Reclaimable: ", Style::default().fg(Color::White)),
        Span::styled(format_bytes(reclaimable), Style::default().fg(Color::Cyan)),
        Span::raw(" | "),
        Span::styled("Selected: ", Style::default().fg(Color::White)),
        Span::styled(format_bytes(selected), Style::default().fg(Color::Yellow)),
    ]);

    let block = Block::default()
        .title(" Disk Summary ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let paragraph = Paragraph::new(vec![line1, line2, line3]).block(block);
    frame.render_widget(paragraph, area);
}
