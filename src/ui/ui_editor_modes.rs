use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::core::{command_line::CommandLine, Core};
/// Rendu du mode [`InputMode::Normal`](enum@crate::app::InputMode).
pub fn ui_normal_mode(frame: &mut Frame, area: Rect) {
    footer_line(
        frame,
        area,
        " NORMAL MODE ",
        Style::default().bold().fg(Color::Black).bg(Color::Yellow),
        None,
    );
}

/// Rendu du mode [`InputMode::Insert`](enum@crate::app::InputMode).
pub fn ui_insert_mode(frame: &mut Frame, area: Rect) {
    footer_line(
        frame,
        area,
        " INSERT MODE ",
        Style::default().bold().fg(Color::Black).bg(Color::Blue),
        None,
    );
}

/// Rendu du mode [`InputMode::Command`](enum@crate::app::InputMode).
pub fn ui_command_mode(app: &mut Core, frame: &mut Frame, area: Rect) {
    let mut command_line = &mut app.command_line;
    footer_line(
        frame,
        area,
        " COMMAND MODE ",
        Style::default().bold().fg(Color::Black).bg(Color::Green),
        Some(&mut command_line),
    );
}

/// Cette fonction va très certainement changer de nom. Rendu de la ligne en bas du terminal.
pub fn footer_line(
    frame: &mut Frame,
    area: Rect,
    text: &str,
    style: Style,
    command_line: Option<&mut CommandLine>,
) {
    let command_line = if let Some(cmd) = command_line {
        cmd
    } else {
        &mut CommandLine::new()
    };
    let padding =
        " ".repeat(area.width as usize - (text.len() + command_line.command_buffer.len()));
    let line = Line::from(vec![
        Span::styled(
            command_line.command_buffer.as_str(),
            Style::default().fg(Color::White),
        ),
        Span::from(padding),
        Span::styled(text, style),
    ])
    .style(Style::default().bg(Color::Rgb(0x2f, 0x32, 0x42)));
    frame.render_widget(Paragraph::new(line), area);
}
