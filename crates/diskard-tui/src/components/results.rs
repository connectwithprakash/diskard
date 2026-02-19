use diskard_core::finding::RiskLevel;
use diskard_core::size::format_bytes;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::Frame;

use crate::app::App;

/// Render the results list.
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
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
