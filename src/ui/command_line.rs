use std::rc::Rc;

use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Span,
    widgets::Paragraph,
    Frame,
};

/// Cette fonction va très certainement changer de nom. Rendu de la ligne en bas du terminal.
pub fn draw(
    frame: &mut Frame,
    text: &str,
    style: Style,
    chunks: Rc<[Rect]>,
) {
    frame.render_widget(Paragraph::default().bg(Color::Red), chunks[0]);
    frame.render_widget(Span::styled(text, style), chunks[1]);
}
