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
    pub text_buffer: Vec<u8>,
    pub scroll: (u16, u16)
}

impl App {
    /// Crée une nouvelle instance de [`App`].
    pub fn new() -> Self {
        let text = "alex
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
papa
".as_bytes();
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(text);

        Self {
            input_mode: InputMode::Normal,
            command_line: CommandLine::new(),
            text_buffer: buffer,
            scroll: (0,0)
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
