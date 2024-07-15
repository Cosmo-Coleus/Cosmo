use ratatui::{layout::Rect, style::{Color, Style, Stylize}, text::{Line, Span}, widgets::Paragraph, Frame};
use crate::core::Core;

pub fn draw(frame: &mut Frame, core: &mut Core, area: Rect) {
    let text = std::str::from_utf8(&core.editor.text_buffer).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    let mut new_lines = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        new_lines.push(Line::from(vec![
            Span::styled(
                format!("  {:<4} ", i + 1),
                Style::default().fg(Color::Rgb(0x7e, 0x82, 0x94)),
            ),
            Span::styled(line.to_string(), Style::default().fg(Color::White)),
        ]));
    }

    frame.render_widget(
        Paragraph::new(new_lines)
            .bg(Color::Rgb(0x2b, 0x2d, 0x3a))
            .scroll(core.editor.scroll),
        area,
    );
}
