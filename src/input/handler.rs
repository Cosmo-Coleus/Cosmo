use ratatui::crossterm::event::KeyCode;

use crate::core::{editor_modes::{command_mode, insert_mode, normal_mode}, Core, InputMode};

pub fn handler_input(key: KeyCode, core: &mut Core) {
    let editor_mode = &core.input_mode;
    match editor_mode {
        InputMode::Normal => handler_input_nomal_mode(key, core),
        InputMode::Insert => handler_input_insert_mode(key, core),
        InputMode::Command => handler_input_command_mode(key, core),
        InputMode::Exit => {}
    }
}

fn handler_input_nomal_mode(key: KeyCode, core: &mut Core) {
    normal_mode(core, key);
}

fn handler_input_insert_mode(key: KeyCode, core: &mut Core) {
   insert_mode(core, key);
}

fn handler_input_command_mode(key: KeyCode, core: &mut Core) {
    command_mode(core, key);
}
