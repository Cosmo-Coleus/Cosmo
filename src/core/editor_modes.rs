use ratatui::crossterm::event::KeyCode;

use super::{command_line::CommandLine, commands::{command_invoker::CommandInvoker, scroll_command::{ScrollDownCommand, ScrollUpCommand}, set_mode_command::{SetCommandMode, SetExitMode, SetInsertMode, SetNormalMode}}};

/// Récupère toutes les touches pressées en [`InputMode::Normal`] et les associe au comportement attendu.
pub fn normal_mode(key: KeyCode, invoker: &mut CommandInvoker) {
    match key {
        KeyCode::Char('i') => invoker.execute_command(SetInsertMode),
        KeyCode::Char('q') => invoker.execute_command(SetExitMode),
        KeyCode::Char(':') => {
            invoker.execute_command(SetCommandMode)
            //app.command_line.command_buffer.push(':');
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
pub fn command_mode(key: KeyCode, invoker: &mut CommandInvoker, command_line: &mut CommandLine) {
    match key {
        KeyCode::Char(ch) => {
            command_line.add_char_in_command_line(ch)
        }
        KeyCode::Backspace => {
            command_line.remove_char_in_command_line()
        }
        KeyCode::Enter => {
            invoker.execute_command(SetNormalMode)
            //check_cmd(app);
            //app.command_line.command_buffer = "".to_string();
        }
        KeyCode::Esc => {
            invoker.execute_command(SetNormalMode)
            //app.command_line.command_buffer = "".to_string();
        }
        _ => {}
    }
}
