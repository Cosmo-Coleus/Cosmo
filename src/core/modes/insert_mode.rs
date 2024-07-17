use ratatui::crossterm::event::KeyCode;

use crate::core::{commands::Commands, queue::CommandQueue};

/// Récupère toutes les touches pressées en [`InputMode::Insert`] et les associent au comportement attendu.
pub fn insert_mode(key: KeyCode, queue: &mut CommandQueue) {
    if key == KeyCode::Esc {
        queue.push_cmd(Commands::SetNormalMode);
    }
}
