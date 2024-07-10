use crate::ui::ui;
use commands::CommandLine;
use editor_modes::{command_mode, insert_mode, normal_mode};
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyEvent},
    Terminal,
};
use std::io::Result;
pub mod commands;
mod editor_modes;

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

    fn handle_modes(self: &mut App, key: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => normal_mode(self, key.code),
            InputMode::Insert => insert_mode(self, key.code),
            InputMode::Command => command_mode(self, key.code),
            _ => {}
        }
    }

    pub fn run_app<B: Backend>(self: &mut App, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|frame| ui(frame, self))?;
            if let Event::Key(key) = event::read()? {
                self.handle_modes(key);
            }
            if self.input_mode == InputMode::Exit {
                break;
            }
        }
        Ok(())
    }
}
