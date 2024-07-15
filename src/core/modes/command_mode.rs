use ratatui::crossterm::event::KeyCode;

use crate::core::{commands::Commands, queue::CommandsQueue};

/// Récupère toutes les touches pressées en [`InputMode::Command`] et les associe au comportement attendu.
pub fn command_mode(key: KeyCode, queue: &mut CommandsQueue) {
    match key {
        KeyCode::Char(ch) => queue.push_cmd(Commands::WriteChar(ch)),
        KeyCode::Backspace => queue.push_cmd(Commands::RemoveChar),
        KeyCode::Enter => {
            queue.push_cmd(Commands::CleanBuffer);
            queue.push_cmd(Commands::SetNormalMode);
            queue.push_cmd(Commands::RunCmdLine);
        }
        KeyCode::Esc => {
            queue.push_cmd(Commands::CleanBuffer);
            queue.push_cmd(Commands::SetNormalMode);
        }
        _ => {}
    }
}
