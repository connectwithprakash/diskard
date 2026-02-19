use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

/// Render a help overlay.
pub fn render(frame: &mut Frame, area: Rect) {
    let popup_width = 40.min(area.width.saturating_sub(4));
    let popup_height = 15.min(area.height.saturating_sub(2));
    let popup_area = Rect::new(
        (area.width - popup_width) / 2,
        (area.height - popup_height) / 2,
        popup_width,
        popup_height,
    );

    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(" j/↓ ", Style::default().fg(Color::Cyan)),
            Span::raw("Move down"),
        ]),
        Line::from(vec![
            Span::styled(" k/↑ ", Style::default().fg(Color::Cyan)),
            Span::raw("Move up"),
        ]),
        Line::from(vec![
            Span::styled(" Space ", Style::default().fg(Color::Cyan)),
            Span::raw("Toggle selection"),
        ]),
        Line::from(vec![
            Span::styled(" a ", Style::default().fg(Color::Cyan)),
            Span::raw("Select/deselect all"),
        ]),
        Line::from(vec![
            Span::styled(" l/→ ", Style::default().fg(Color::Cyan)),
            Span::raw("Inspect directory"),
        ]),
        Line::from(vec![
            Span::styled(" d ", Style::default().fg(Color::Cyan)),
            Span::raw("Delete selected (in inspect)"),
        ]),
        Line::from(vec![
            Span::styled(" Enter ", Style::default().fg(Color::Cyan)),
            Span::raw("Delete selected / open dir"),
        ]),
        Line::from(vec![
            Span::styled(" ? ", Style::default().fg(Color::Cyan)),
            Span::raw("Toggle help"),
        ]),
        Line::from(vec![
            Span::styled(" q/Esc ", Style::default().fg(Color::Cyan)),
            Span::raw("Quit"),
        ]),
    ];

    frame.render_widget(Clear, popup_area);
    let paragraph = Paragraph::new(text).alignment(Alignment::Left).block(
        Block::default()
            .title(" Help ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    frame.render_widget(paragraph, popup_area);
}
