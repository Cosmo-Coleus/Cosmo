use crate::core::{
    commands::command_invoker::CommandInvoker, modes::{command_mode::command_mode, insert_mode::insert_mode, normal_mode::normal_mode}, Core, InputMode
};
use ratatui::crossterm::event::KeyCode;

/// Recupere les KeyCode pour ensuite les traiter en fonction du context ([`InputMode`](enum@InputMode))
pub fn handler_input(key: KeyCode, core: &mut Core) {
    let input_mode = core.editor.input_mode;
    let mut invoker = CommandInvoker::new(core);
    match input_mode {
        InputMode::Normal => handler_input_nomal_mode(key, &mut invoker),
        InputMode::Insert => handler_input_insert_mode(key, &mut invoker),
        InputMode::Command => handler_input_command_mode(key, &mut invoker),
        InputMode::Exit => {}
    }
}

fn handler_input_nomal_mode(key: KeyCode, invoker: &mut CommandInvoker) {
    normal_mode(key, invoker);
}

fn handler_input_insert_mode(key: KeyCode, invoker: &mut CommandInvoker) {
    insert_mode(key, invoker);
}

fn handler_input_command_mode(key: KeyCode, invoker: &mut CommandInvoker) {
    command_mode(key, invoker);
}
