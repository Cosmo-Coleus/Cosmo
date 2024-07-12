use command_line::CommandLine;
use editor::Editor;
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event},
    Terminal,
};
use std::io::Result;
use crate::{input::handler::handler_input, ui::ui};

/// Gestion de toutes les actions possible dans **Cosmo**
pub mod commands;
/// Gestion de l'editeur **Cosmo**
mod editor;
pub mod command_line;
pub mod modes;
mod utils;


/// Liste les différents modes d'interaction de l'IDE.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum InputMode {
    Normal,
    Insert,
    Command,
    Exit,
}

/// Representation de tous les modules de **Cosmo**
pub struct Core {
    pub editor: Editor,
    pub command_line: CommandLine,
}

impl Core {
    /// Crée une nouvelle instance de [`Core`].
    pub fn new() -> Self {
        Self {
            editor: Editor::new(),
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
            if self.editor.input_mode == InputMode::Exit {
                break;
            }
        }
        Ok(())
    }
}
