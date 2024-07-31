use std::{io::Result, time::Duration};
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event},
    Terminal,
};
use crate::{input::handler::handler_input, ui::ui};
use command_line::CommandLine;
use commands::command_invoker::CommandInvoker;
use editor::Editor;
use queue::CommandQueue;

pub mod command_line;
/// Gestion de toutes les actions possibles dans **Cosmo**
pub mod commands;
/// Gestion de l'éditeur **Cosmo**
mod editor;
pub mod modes;
pub mod queue;

/// Liste les différents modes d'interaction de l'IDE.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum InputMode {
    Normal,
    Insert,
    Command,
    Exit,
}

/// Représentation de tous les modules de **Cosmo**
pub struct Core {
    pub editor: Editor,
    pub command_line: CommandLine,
    pub queue: CommandQueue,
}

impl Core {
    /// Créée une nouvelle instance de [`Core`].
    pub fn new() -> Self {
        Self {
            editor: Editor::new(),
            command_line: CommandLine::new(),
            queue: CommandQueue::new(),
        }
    }

    /// Contient la boucle de rendu et d'évèvement de **Cosmo**.
    pub fn run_app<B: Backend>(self: &mut Core, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|frame| ui(frame, self))?;
            if event::poll(Duration::from_micros(16))? {
                if let Event::Key(key) = event::read()? {
                    handler_input(key.code, &mut self.queue, &self.editor.input_mode);
                }
            }

            let mut invoker = CommandInvoker::new(self);

            invoker.run_cmd();
            if self.editor.input_mode == InputMode::Exit {
                break;
            }
        }
        Ok(())
    }
}
