use diskard_core::finding::RiskLevel;
use diskard_core::size::format_bytes;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::Frame;

use crate::app::App;

const BAR_WIDTH: usize = 10;

fn size_bar(size: u64, max_size: u64) -> String {
    if max_size == 0 {
        return " ".repeat(BAR_WIDTH);
    }
    let ratio = (size as f64 / max_size as f64).clamp(0.0, 1.0);
    let filled = ratio * BAR_WIDTH as f64;
    let full_blocks = filled as usize;
    let remainder = filled - full_blocks as f64;

    let mut bar = "█".repeat(full_blocks.min(BAR_WIDTH));
    if full_blocks < BAR_WIDTH {
        const EIGHTHS: [char; 9] = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];
        let idx = (remainder * 8.0) as usize;
        bar.push(EIGHTHS[idx.min(8)]);
        for _ in (full_blocks + 1)..BAR_WIDTH {
            bar.push(' ');
        }
    }
    bar
}

/// Render the results list.
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let max_size = app
        .findings
        .iter()
        .map(|f| f.finding.size_bytes)
        .max()
        .unwrap_or(0);

    let items: Vec<ListItem> = app
        .findings
        .iter()
        .map(|item| {
            let checkbox = if item.checked { "[x]" } else { "[ ]" };
            let risk_color = match item.finding.risk {
                RiskLevel::Safe => Color::Green,
                RiskLevel::Moderate => Color::Yellow,
                RiskLevel::Risky => Color::Red,
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!(" {checkbox} "), Style::default().fg(Color::White)),
                Span::styled(
                    format!("{:>10}", item.finding.size_human()),
                    Style::default().fg(Color::Cyan),
                ),
                Span::raw(" "),
                Span::styled(
                    size_bar(item.finding.size_bytes, max_size),
                    Style::default().fg(Color::Cyan),
                ),
                Span::raw("  "),
                Span::styled(
                    format!("{:>8}", item.finding.risk),
                    Style::default().fg(risk_color),
                ),
                Span::raw("  "),
                Span::raw(&item.finding.description),
            ]))
        })
        .collect();

    let title = format!(
        " {} items | {} selected | {} ",
        app.findings.len(),
        app.checked_count(),
        format_bytes(app.checked_size()),
    );

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        );

    let mut state = ListState::default();
    state.select(Some(app.selected));
    frame.render_stateful_widget(list, area, &mut state);
}
