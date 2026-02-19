use diskard_core::size::format_bytes;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::app::App;

/// Render a deletion confirmation dialog.
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let (count, size) = if let Some(ref state) = app.drill_down {
        (state.checked_count(), state.checked_size())
    } else {
        (app.checked_count(), app.checked_size())
    };

    render_dialog(frame, area, count, size);
}

fn render_dialog(frame: &mut Frame, area: Rect, count: usize, size: u64) {
    let popup_width = 50.min(area.width.saturating_sub(4));
    let popup_height = 7.min(area.height.saturating_sub(2));
    let popup_area = Rect::new(
        (area.width - popup_width) / 2,
        (area.height - popup_height) / 2,
        popup_width,
        popup_height,
    );

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("Delete {} items ({})?", count, format_bytes(size)),
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(" [y] ", Style::default().fg(Color::Green)),
            Span::raw("Confirm  "),
            Span::styled(" [n] ", Style::default().fg(Color::Red)),
            Span::raw("Cancel"),
        ]),
    ];

    frame.render_widget(Clear, popup_area);
    let paragraph = Paragraph::new(text).alignment(Alignment::Center).block(
        Block::default()
            .title(" Confirm Deletion ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)),
    );
    frame.render_widget(paragraph, popup_area);
}
