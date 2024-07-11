use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::core::Core;

/// Gère le rendu graphique de l'éditeur de texte
pub fn ui_editor_view(app: &mut Core, frame: &mut Frame, area: Rect) {
    let text = std::str::from_utf8(&app.editor.text_buffer).unwrap();
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
            .scroll(app.editor.scroll),
        area,
    );
}
