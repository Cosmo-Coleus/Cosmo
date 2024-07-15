use crate::core::{
    modes::{command_mode::command_mode, insert_mode::insert_mode, normal_mode::normal_mode},
    queue::CommandsQueue,
    InputMode,
};
use ratatui::crossterm::event::KeyCode;

/// Recupere les KeyCode pour ensuite les traiter en fonction du context ([`InputMode`](enum@InputMode))
pub fn handler_input(key: KeyCode, queue: &mut CommandsQueue, input_mode: &InputMode) {
    match input_mode {
        InputMode::Normal => handler_input_nomal_mode(key, queue),
        InputMode::Insert => handler_input_insert_mode(key, queue),
        InputMode::Command => handler_input_command_mode(key, queue),
        InputMode::Exit => {}
    }
}

fn handler_input_nomal_mode(key: KeyCode, queue: &mut CommandsQueue) {
    normal_mode(key, queue);
}

fn handler_input_insert_mode(key: KeyCode, queue: &mut CommandsQueue) {
    insert_mode(key, queue);
}

fn handler_input_command_mode(key: KeyCode, queue: &mut CommandsQueue) {
    command_mode(key, queue);
}
