use ratatui::crossterm::event::KeyCode;

use super::{commands::check_cmd, App, InputMode};

pub fn normal_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('i') => app.input_mode = InputMode::Insert,
        KeyCode::Char(':') => {
            app.input_mode = InputMode::Command;
            app.command_line.command_buffer.push(':');
        }
        _ => {}
    }
}

pub fn insert_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => app.input_mode = InputMode::Normal,
        _ => {}
    }
}

pub fn command_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char(insert) => {
            app.command_line.command_buffer.push(insert);
        }
        KeyCode::Enter => {
            app.input_mode = InputMode::Normal;
            check_cmd(app);
            app.command_line.command_buffer = "".to_string();
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.command_line.command_buffer = "".to_string();
        }
        _ => {}
    }
}
