use diskard_core::size::format_bytes;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::Frame;

use crate::app::DrillDownState;

/// Render the drill-down directory listing.
pub fn render(frame: &mut Frame, area: Rect, state: &DrillDownState) {
    let items: Vec<ListItem> = state
        .entries
        .iter()
        .map(|entry| {
            let checkbox = if entry.checked { "[x]" } else { "[ ]" };
            let (name_display, name_color) = if entry.is_dir {
                (format!("{}/", entry.name), Color::Blue)
            } else {
                (entry.name.clone(), Color::White)
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!(" {checkbox} "), Style::default().fg(Color::White)),
                Span::styled(
                    format!("{:>10}", format_bytes(entry.size_bytes)),
                    Style::default().fg(Color::Cyan),
                ),
                Span::raw("  "),
                Span::styled(name_display, Style::default().fg(name_color)),
            ]))
        })
        .collect();

    let current = state.current_path();
    let breadcrumb = current.to_string_lossy();
    let checked = state.checked_count();
    let title = if checked > 0 {
        format!(
            " {} | {} entries | {} selected | {} ",
            breadcrumb,
            state.entries.len(),
            checked,
            format_bytes(state.checked_size()),
        )
    } else {
        format!(
            " {} | {} entries | {} ",
            breadcrumb,
            state.entries.len(),
            format_bytes(state.total_size()),
        )
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        );

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected));
    frame.render_stateful_widget(list, area, &mut list_state);
}
