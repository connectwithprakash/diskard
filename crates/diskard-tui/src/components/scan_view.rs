use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

/// Render a scanning progress indicator.
pub fn render(frame: &mut Frame, area: Rect, elapsed_secs: f64) {
    let dots = ".".repeat(((elapsed_secs * 2.0) as usize % 4) + 1);
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  Scanning{dots}"),
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  Elapsed: {elapsed_secs:.1}s"),
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let block = Block::default()
        .title(" diskard ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}
