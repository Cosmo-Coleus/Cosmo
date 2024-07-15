use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::core::{command_line::CommandLine, cursor::Cursor};


/// Cette fonction va très certainement changer de nom. Rendu de la ligne en bas du terminal.
pub fn draw(
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
    let padding = " ".repeat(area.width as usize - (text.len() + command_line.buffer.len()));
    let line = Line::from(vec![
        Span::styled(
            command_line.buffer.as_str(),
            Style::default().fg(Color::White),
        ),
        Span::from(padding),
        Span::styled(text, style),
    ])
    .style(Style::default().bg(Color::Rgb(0x2f, 0x32, 0x42)));
    frame.render_widget(Paragraph::new(line), area);
}
