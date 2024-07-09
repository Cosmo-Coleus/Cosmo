use std::io::Result;


use ratatui::{backend::Backend, crossterm::event::{self, Event, KeyCode, KeyEvent}, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, text::{Line, Span}, widgets::{Block, Paragraph}, Frame, Terminal};

#[derive(PartialEq, Eq)]
enum InputMode {
    Normal,
    Insert,
    Command,
    Exit
}

pub struct App {
    input_mode: InputMode,
    command_line: CommandLine
}

impl App {
    pub const fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
            command_line: CommandLine::new()
        }
    }
    fn manage_input_mode(self: &mut App, key: KeyEvent)  {
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('i') => self.input_mode = InputMode::Insert,
                KeyCode::Char(':') => {
                    self.input_mode = InputMode::Command;
                    self.command_line.commande_buffer.push(':');
                },
                _ => {}
            },
            InputMode::Insert => match key.code {
                KeyCode::Esc => self.input_mode = InputMode::Normal,
                _ => {}
            },
            InputMode::Command => match key.code {
                KeyCode::Char(insert) => {
                    self.command_line.commande_buffer.push(insert);
                },
                KeyCode::Enter => {
                    self.input_mode = InputMode::Normal;
                    check_cmd(self);
                    self.command_line.commande_buffer = "".to_string();
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.command_line.commande_buffer = "".to_string();
                },
                _ => {}
            },
            _ => {}
        }
    }
}

struct CommandLine {
    commande_buffer: String
}

impl CommandLine {
    const fn new() -> Self {
        Self { commande_buffer: String::new() }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            app.manage_input_mode(key);
        }
        if app.input_mode == InputMode::Exit {
            break;
        }
    }
    Ok(())
}

fn ui(frame: &mut Frame, app: &mut App) {
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(100),
            Constraint::Length(1)
        ])
        .split(frame.size());

    match app.input_mode {
        InputMode::Normal => {
            footer_line(frame, area[1], " NORMAL MODE ", Style::default().bold().fg(Color::Black).bg(Color::Yellow), None)
        },
        InputMode::Insert => {
            footer_line(frame, area[1], " INSERT MODE ", Style::default().bold().fg(Color::Black).bg(Color::Blue), None)
        },
        InputMode::Command => {
            let mut command_line = &mut app.command_line;
            footer_line(frame, area[1], " COMMAND MODE ", Style::default().bold().fg(Color::Black).bg(Color::Green), Some(&mut command_line))
        },
        InputMode::Exit => {}
    };

    frame.render_widget(
        Block::default(),
        area[0]
    );
}

fn footer_line(frame: &mut Frame, area: Rect, text: &str, style: Style, command_line: Option<&mut CommandLine>) {
    let command_line = if let Some(cmd) = command_line {
        cmd
    } else {
        &mut CommandLine::new()
    };
    let padding = " ".repeat(area.width as usize - (text.len() + command_line.commande_buffer.len()));
    let line = Line::from(vec![
       Span::styled(command_line.commande_buffer.as_str() , Style::default().fg(Color::White)),
       Span::from(padding),
       Span::styled(text, style)
    ]);
    frame.render_widget(
        Paragraph::new(line),
        area
    );
}

fn check_cmd(app: &mut App) {
    let cmd = &app.command_line.commande_buffer;
    if cmd == ":q" {
        app.input_mode = InputMode::Exit;
    }
}
