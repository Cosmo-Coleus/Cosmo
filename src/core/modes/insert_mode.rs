use ratatui::crossterm::event::KeyCode;

use crate::core::commands::command_invoker::CommandInvoker;

/// Récupère toutes les touches pressées en [`InputMode::Insert`] et les associe au comportement attendu.
pub fn insert_mode(key: KeyCode, invoker: &mut CommandInvoker) {
    if key == KeyCode::Esc {
        //invoker.execute_command(SetNormalMode)
    }
}
