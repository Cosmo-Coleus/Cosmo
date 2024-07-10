use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::app::{commands::CommandLine, App, InputMode};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100), Constraint::Length(1)])
        .split(frame.size());

    match app.input_mode {
        InputMode::Normal => footer_line(
            frame,
            area[1],
            " NORMAL MODE ",
            Style::default().bold().fg(Color::Black).bg(Color::Yellow),
            None,
        ),
        InputMode::Insert => footer_line(
            frame,
            area[1],
            " INSERT MODE ",
            Style::default().bold().fg(Color::Black).bg(Color::Blue),
            None,
        ),
        InputMode::Command => {
            let mut command_line = &mut app.command_line;
            footer_line(
                frame,
                area[1],
                " COMMAND MODE ",
                Style::default().bold().fg(Color::Black).bg(Color::Green),
                Some(&mut command_line),
            )
        }
        InputMode::Exit => {}
    };

    frame.render_widget(Block::default(), area[0]);
}

fn footer_line(
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
    ]);
    frame.render_widget(Paragraph::new(line), area);
}
