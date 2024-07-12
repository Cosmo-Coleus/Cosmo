use ratatui::crossterm::event::KeyCode;

use super::commands::{command_invoker::CommandInvoker, scroll_command::{ScrollDownCommand, ScrollUpCommand}, set_mode_command::{SetCommandMode, SetExitMode, SetInsertMode, SetNormalMode}, write_cmd_line_command::{CleanBuffer, RemoveChar, WriteChar}};

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

/// Récupère toutes les touches pressées en [`InputMode::Insert`] et les associe au comportement attendu.
pub fn insert_mode(key: KeyCode, invoker: &mut CommandInvoker) {
    if key == KeyCode::Esc { invoker.execute_command(SetNormalMode) }
}

/// Récupère toutes les touches pressées en [`InputMode::Command`] et les associe au comportement attendu.
pub fn command_mode(key: KeyCode, invoker: &mut CommandInvoker) {
    match key {
        KeyCode::Char(ch) => {
            invoker.execute_command(WriteChar(ch))
        }
        KeyCode::Backspace => {
            invoker.execute_command(RemoveChar)
        }
        KeyCode::Enter => {
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
