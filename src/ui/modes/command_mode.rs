use std::rc::Rc;

use ratatui::{layout::Rect, style::{Color, Style, Stylize}, Frame};

use crate::{core::Core, ui::command_line};

/// Rendu du mode [`InputMode::Command`](enum@crate::app::InputMode).
pub fn draw(core: &mut Core, frame: &mut Frame, chunks: Rc<[Rect]>) {
    command_line::draw(
        frame,
        " COMMAND MODE ",
        Style::default().bold().fg(Color::Black).bg(Color::Green),
        chunks,
    );
}
