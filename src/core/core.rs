use std::io::Result;

use ratatui::{backend::Backend, crossterm::event::{self, Event, KeyEvent}, Terminal};

use crate::ui::ui;

use super::{commands::CommandLine, editor::Editor, editor_modes::{command_mode, insert_mode, normal_mode}, InputMode};

/// Representation des donnees de **Cosmo**
pub struct Core {
    pub editor: Editor,
    pub input_mode: InputMode,
    pub command_line: CommandLine,
}

impl Core {
    /// Crée une nouvelle instance de [`App`].
    pub fn new() -> Self {
        Self {
            editor: Editor::new(),
            input_mode: InputMode::Normal,
            command_line: CommandLine::new(),
        }
    }


    /// # Warning
    /// Cette fonction est temporaire et sera très certainement supprimé dans le futur.
    fn handle_modes(self: &mut Core, key: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => normal_mode(self, key.code),
            InputMode::Insert => insert_mode(self, key.code),
            InputMode::Command => command_mode(self, key.code),
            _ => {}
        }
    }

    /// Contient la boucle de rendu et d'évévement de **Cosmo**.
    pub fn run_app<B: Backend>(self: &mut Core, terminal: &mut Terminal<B>) -> Result<()> {
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
