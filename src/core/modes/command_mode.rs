use ratatui::crossterm::event::KeyCode;

use crate::core::commands::{command_invoker::CommandInvoker, run_cmd_line_command::RunCmdLine, set_mode_command::SetNormalMode, write_cmd_line_command::{CleanBuffer, RemoveChar, WriteChar}};

/// Récupère toutes les touches pressées en [`InputMode::Command`] et les associe au comportement attendu.
pub fn command_mode(key: KeyCode, invoker: &mut CommandInvoker) {
    match key {
        KeyCode::Char(ch) => invoker.execute_command(WriteChar(ch)),
        KeyCode::Backspace => invoker.execute_command(RemoveChar),
        KeyCode::Enter => {
            invoker.execute_command(RunCmdLine);
            invoker.execute_command(SetNormalMode);
            invoker.execute_command(CleanBuffer);
        }
        KeyCode::Esc => {
            invoker.execute_command(SetNormalMode);
            invoker.execute_command(CleanBuffer);
        }
        _ => {}
    }
}
