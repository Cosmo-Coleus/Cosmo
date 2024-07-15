use ratatui::crossterm::event::KeyCode;

use crate::core::{commands::Commands, queue::CommandsQueue};

/// Récupère toutes les touches pressées en [`InputMode::Normal`] et les associe au comportement attendu.
pub fn normal_mode(key: KeyCode, queue: &mut CommandsQueue) {
    match key {
        KeyCode::Char('i') => queue.push_cmd(Commands::SetInsertMode),
        KeyCode::Char('q') => queue.push_cmd(Commands::SetExitMode),
        KeyCode::Char(':') => {
            queue.push_cmd(Commands::SetCommandMode);
            queue.push_cmd(Commands::WriteChar(':'));
        }
        KeyCode::Down => queue.push_cmd(Commands::ScrollUp),
        KeyCode::Up => queue.push_cmd(Commands::ScrollDown),
        _ => {}
    }
}
