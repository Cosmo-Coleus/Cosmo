use crate::{input::handler::handler_input, ui::draw};
use command_line::CommandLine;
use commands::command_invoker::CommandInvoker;
use cursor::Cursor;
use editor::Editor;
use queue::CommandsQueue;
use ratatui::{
    backend::Backend, crossterm::event::{self, Event}, Frame, Terminal
};
use std::{io::Result, time::Duration};

pub mod command_line;
/// Gestion de toutes les actions possible dans **Cosmo**
pub mod commands;
/// Gestion de l'editeur **Cosmo**
mod editor;
pub mod modes;
pub mod queue;
pub mod cursor;

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
    pub queue: CommandsQueue,
    pub cursor: Cursor
}

impl Core {
    /// Crée une nouvelle instance de [`Core`].
    pub fn new() -> Self {
        Self {
            editor: Editor::new(),
            command_line: CommandLine::new(),
            queue: CommandsQueue::new(),
            cursor: Cursor::new(0, 0)
        }
    }

    pub fn set_cursor(&mut self, frame: &mut Frame) {
        let mode = self.editor.input_mode;

        match mode {
            InputMode::Normal => self.cursor.set_cursor(frame),
            InputMode::Insert => self.cursor.set_cursor(frame),
            InputMode::Command => self.cursor.set_cursor(frame),
            InputMode::Exit => {},
        }
    }

    pub fn set_cursor_pos(&mut self, x: u16, y: u16) {
        self.cursor.set_pos(x, y);
    }

    /// Contient la boucle de rendu et d'évévement de **Cosmo**.
    pub fn run_app<B: Backend>(self: &mut Core, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|frame| draw(frame, self))?;
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
