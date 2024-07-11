/// Gestion de l'editeur **Cosmo**
mod editor;

use commands::CommandLine;
use editor::Editor;
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event},
    Terminal,
};
use std::io::Result;

use crate::{input::handler::handler_input, ui::ui};
pub mod commands;
pub mod editor_modes;
mod editor_view;
mod utils;

/// Liste les différents modes d'interaction de l'IDE.
#[derive(PartialEq, Eq, Debug)]
pub enum InputMode {
    Normal,
    Insert,
    Command,
    Exit,
}

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

    /// Contient la boucle de rendu et d'évévement de **Cosmo**.
    pub fn run_app<B: Backend>(self: &mut Core, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|frame| ui(frame, self))?;
            if let Event::Key(key) = event::read()? {
                handler_input(key.code, self)
            }
            if self.input_mode == InputMode::Exit {
                break;
            }
        }
        Ok(())
    }
}
