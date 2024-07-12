use ratatui::crossterm::event::KeyCode;

use crate::core::commands::{command_invoker::CommandInvoker, scroll_command::{ScrollDownCommand, ScrollUpCommand}, set_mode_command::{SetCommandMode, SetExitMode, SetInsertMode}, write_cmd_line_command::WriteChar};

/// Récupère toutes les touches pressées en [`InputMode::Normal`] et les associe au comportement attendu.
pub fn normal_mode(key: KeyCode, invoker: &mut CommandInvoker) {
    match key {
        KeyCode::Char('i') => invoker.execute_command(SetInsertMode),
        KeyCode::Char('q') => invoker.execute_command(SetExitMode),
        KeyCode::Char(':') => {
            invoker.execute_command(SetCommandMode);
            invoker.execute_command(WriteChar(':'));
        }
        KeyCode::Down => invoker.execute_command(ScrollUpCommand),
        KeyCode::Up => invoker.execute_command(ScrollDownCommand),
        _ => {}
    }
}
