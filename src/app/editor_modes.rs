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

#[cfg(test)]
mod test {
    use ratatui::crossterm::event::KeyCode;
    use crate::app::{editor_modes::{command_mode, insert_mode, normal_mode}, App, InputMode};

    fn setup() -> App {
        App::new()
    }

    #[test]
    fn test_normal_mode_to_insert_mode() {
        let mut app = setup();
        normal_mode(&mut app, KeyCode::Char('i'));
        assert_eq!(app.input_mode, InputMode::Insert);
    }

    #[test]
    fn test_normal_mode_to_command_mode() {
        let mut app = setup();
        normal_mode(&mut app, KeyCode::Char(':'));
        assert_eq!(app.input_mode, InputMode::Command);
    }

    #[test]
    fn test_insert_mode_to_normal_mode() {
        let mut app = setup();
        normal_mode(&mut app, KeyCode::Char('i'));
        insert_mode(&mut app, KeyCode::Esc);
        assert_eq!(app.input_mode, InputMode::Normal);
    }

    #[test]
    fn test_command_mode_to_normal_mode() {
        let mut app = setup();
        normal_mode(&mut app, KeyCode::Char(':'));
        command_mode(&mut app, KeyCode::Esc);
        assert_eq!(app.input_mode, InputMode::Normal);
    }

    #[test]
    fn test_command_mode_to_exit() {
        let mut app = setup();
        normal_mode(&mut app, KeyCode::Char(':'));
        command_mode(&mut app, KeyCode::Char('q'));
        command_mode(&mut app, KeyCode::Enter);
        assert_eq!(app.input_mode, InputMode::Exit);
    }
}
