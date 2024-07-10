pub mod commands;
mod editor_modes;

use crate::ui::ui;
use commands::CommandLine;
use editor_modes::{command_mode, insert_mode, normal_mode};
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyEvent},
    Terminal,
};
use std::io::Result;

/// Liste les différents modes d'interaction de l'IDE.
#[derive(PartialEq, Eq, Debug)]
pub enum InputMode {
    Normal,
    Insert,
    Command,
    Exit,
}

/// Gère tout ce qui n'est pas graphique.
pub struct App {
    pub input_mode: InputMode,
    pub command_line: CommandLine,
}

impl App {
    /// Crée une nouvelle instance de [`App`].
    pub const fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
            command_line: CommandLine::new(),
        }
    }

    /// # Warning
    /// Cette fonction est temporaire et sera très certainement supprimé dans le futur.
    fn handle_modes(self: &mut App, key: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => normal_mode(self, key.code),
            InputMode::Insert => insert_mode(self, key.code),
            InputMode::Command => command_mode(self, key.code),
            _ => {}
        }
    }

    /// Contient la boucle de rendu et d'évévement de **Cosmo**.
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
