use ratatui::crossterm::event::KeyCode;

use super::{commands::check_cmd, Core, InputMode};

/// Récupère toutes les touches pressées en [`InputMode::Normal`] et les associe au comportement attendu.
pub fn normal_mode(app: &mut Core, key: KeyCode) {
    match key {
        KeyCode::Char('i') => app.input_mode = InputMode::Insert,
        KeyCode::Char(':') => {
            app.input_mode = InputMode::Command;
            app.command_line.command_buffer.push(':');
        }
        KeyCode::Down => app.editor.scroll_up(),
        KeyCode::Up => app.editor.scroll_down(),
        _ => {}
    }
}

/// Récupère toutes les touches pressées en [`InputMode::Insert`] et les associe au comportement attendu.
pub fn insert_mode(app: &mut Core, key: KeyCode) {
    if key == KeyCode::Esc {
        app.input_mode = InputMode::Normal
    }
}

/// Récupère toutes les touches pressées en [`InputMode::Command`] et les associe au comportement attendu.
pub fn command_mode(app: &mut Core, key: KeyCode) {
    match key {
        KeyCode::Char(insert) => {
            app.command_line.command_buffer.push(insert);
        }
        KeyCode::Backspace => {
            app.command_line.command_buffer.pop();
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
    use crate::core::{
        editor_modes::{command_mode, insert_mode, normal_mode},
        Core, InputMode,
    };
    use ratatui::crossterm::event::KeyCode;

    fn setup() -> Core {
        Core::new()
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
