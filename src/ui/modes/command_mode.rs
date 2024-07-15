use ratatui::{layout::Rect, style::{Color, Style, Stylize}, Frame};

use crate::{core::command_line::CommandLine, ui::command_line};

/// Rendu du mode [`InputMode::Command`](enum@crate::app::InputMode).
pub fn draw(cmd_line: &mut CommandLine, frame: &mut Frame, area: Rect) {
    command_line::draw(
        frame,
        area,
        " COMMAND MODE ",
        Style::default().bold().fg(Color::Black).bg(Color::Green),
        Some(cmd_line),
    );
    cmd_line.cursor.set_cursor(frame);
}
