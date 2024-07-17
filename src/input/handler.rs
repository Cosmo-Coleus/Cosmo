use crate::core::{
    modes::{command_mode::command_mode, insert_mode::insert_mode, normal_mode::normal_mode},
    queue::CommandQueue,
    InputMode,
};
use ratatui::crossterm::event::KeyCode;

/// Récupère les KeyCode pour ensuite les traiter en fonction du contexte ([`InputMode`](enum@InputMode))
pub fn handler_input(key: KeyCode, queue: &mut CommandQueue, input_mode: &InputMode) {
    match input_mode {
        InputMode::Normal => handler_input_nomal_mode(key, queue),
        InputMode::Insert => handler_input_insert_mode(key, queue),
        InputMode::Command => handler_input_command_mode(key, queue),
        InputMode::Exit => {}
    }
}

fn handler_input_nomal_mode(key: KeyCode, queue: &mut CommandQueue) {
    normal_mode(key, queue);
}

fn handler_input_insert_mode(key: KeyCode, queue: &mut CommandQueue) {
    insert_mode(key, queue);
}

fn handler_input_command_mode(key: KeyCode, queue: &mut CommandQueue) {
    command_mode(key, queue);
}
