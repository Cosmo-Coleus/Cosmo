use crate::ui::ui;
use commands::{check_cmd, CommandLine};
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    Terminal,
};
use std::io::Result;
pub mod commands;

#[derive(PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Insert,
    Command,
    Exit,
}

pub struct App {
    pub input_mode: InputMode,
    pub command_line: CommandLine,
}

impl App {
    pub const fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
            command_line: CommandLine::new(),
        }
    }
    fn manage_input_mode(self: &mut App, key: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('i') => self.input_mode = InputMode::Insert,
                KeyCode::Char(':') => {
                    self.input_mode = InputMode::Command;
                    self.command_line.command_buffer.push(':');
                }
                _ => {}
            },
            InputMode::Insert => match key.code {
                KeyCode::Esc => self.input_mode = InputMode::Normal,
                _ => {}
            },
            InputMode::Command => match key.code {
                KeyCode::Char(insert) => {
                    self.command_line.command_buffer.push(insert);
                }
                KeyCode::Enter => {
                    self.input_mode = InputMode::Normal;
                    check_cmd(self);
                    self.command_line.command_buffer = "".to_string();
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.command_line.command_buffer = "".to_string();
                }
                _ => {}
            },
            _ => {}
        }
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
