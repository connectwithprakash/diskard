use diskard_core::size::format_bytes;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::Frame;

use crate::app::DrillDownState;

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

/// Render the drill-down directory listing.
pub fn render(frame: &mut Frame, area: Rect, state: &DrillDownState) {
    let max_size = state
        .entries
        .iter()
        .map(|e| e.size_bytes)
        .max()
        .unwrap_or(0);

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
                Span::raw(" "),
                Span::styled(
                    size_bar(entry.size_bytes, max_size),
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
