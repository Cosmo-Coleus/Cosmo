use std::io::Result;


use ratatui::{backend::Backend, crossterm::event::{self, Event, KeyCode, KeyEvent}, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, Paragraph}, Frame, Terminal};

#[derive(PartialEq, Eq)]
enum InputMode {
    Normal,
    Insert,
    Command,
    Exit
}

pub struct App {
    input_mode: InputMode
}

impl App {
    pub const fn new() -> Self {
        Self { input_mode: InputMode::Normal }
    }
    fn manage_input_mode(self: &mut App, key: KeyEvent)  {
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('i') => self.input_mode = InputMode::Insert,
                KeyCode::Char(':') => self.input_mode = InputMode::Command,
                KeyCode::Char('q') => self.input_mode = InputMode::Exit,
                _ => {}
            },
            InputMode::Insert => match key.code {
                KeyCode::Esc => self.input_mode = InputMode::Normal,
                _ => {}
            },
            InputMode::Command => match key.code {
                KeyCode::Esc => self.input_mode = InputMode::Normal,
                _ => {}
            },
            _ => {}
        }
    }
}

struct CommandLine {
    commande_buffer: String
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

fn ui(frame: &mut Frame, app: &App) {
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(100),
            Constraint::Length(1)
        ])
        .split(frame.size());

    match app.input_mode {
        InputMode::Normal => {
            footer_line(frame, area[1], " NORMAL MODE ", Style::default().bold().fg(Color::Black).bg(Color::Yellow))
        },
        InputMode::Insert => {
            footer_line(frame, area[1], " INSERT MODE ", Style::default().bold().fg(Color::Black).bg(Color::Blue))
        },
        InputMode::Command => {
            footer_line(frame, area[1], " COMMAND MODE ", Style::default().bold().fg(Color::Black).bg(Color::Green))
        },
        InputMode::Exit => {}
    };

    frame.render_widget(
        Block::default(),
        area[0]
    );
}

fn footer_line(frame: &mut Frame, area: Rect, text: &str, style: Style) {
    let tmp_cmd_buffer = ":commande here";
    let padding = " ".repeat(area.width as usize - (text.len() + tmp_cmd_buffer.len()));
    let line = Line::from(vec![
       Span::styled(tmp_cmd_buffer, Style::default().fg(Color::White)),
       Span::from(padding),
       Span::styled(text, style)
    ]);
    frame.render_widget(
        Paragraph::new(line),
        area
    );
}
