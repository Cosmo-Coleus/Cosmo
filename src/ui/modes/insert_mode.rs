use ratatui::{layout::Rect, style::{Color, Style, Stylize}, Frame};

use crate::ui::command_line;

/// Rendu du mode [`InputMode::Insert`](enum@crate::app::InputMode).
pub fn draw(frame: &mut Frame, area: Rect) {
    command_line::draw(
        frame,
        area,
        " INSERT MODE ",
        Style::default().bold().fg(Color::Black).bg(Color::Blue),
        None,
    );
}
