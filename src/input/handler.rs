use ratatui::crossterm::event::KeyCode;

use crate::core::{command_line::CommandLine, commands::command_invoker::CommandInvoker, editor_modes::{command_mode, insert_mode, normal_mode}, Core, InputMode};

/// Recupere les KeyCode pour ensuite les traiter en fonction du context ([`InputMode`](enum@InputMode))
pub fn handler_input(key: KeyCode, core: &mut Core) {
    let input_mode = core.editor.input_mode;
    let mut invoker = CommandInvoker::new(&mut core.editor);
    match input_mode {
        InputMode::Normal => handler_input_nomal_mode(key, &mut invoker),
        InputMode::Insert => handler_input_insert_mode(key, &mut invoker),
        InputMode::Command => handler_input_command_mode(key, &mut invoker, &mut core.command_line),
        InputMode::Exit => {}
    }
}

fn handler_input_nomal_mode(key: KeyCode, invoker: &mut CommandInvoker) {
    normal_mode(key, invoker);
}

fn handler_input_insert_mode(key: KeyCode, invoker: &mut CommandInvoker) {
   insert_mode(key, invoker);
}

fn handler_input_command_mode(key: KeyCode, invoker: &mut CommandInvoker, command_line: &mut CommandLine) {
    command_mode(key, invoker, command_line);
}
