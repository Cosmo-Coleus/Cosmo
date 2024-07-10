mod ui_editor_modes;
use crate::app::{App, InputMode};
use ratatui::{
    layout::{Constraint, Direction, Layout}, style::{Color, Stylize}, text::Text, widgets::{Block, Paragraph}, Frame
};
use ui_editor_modes::{ui_command_mode, ui_insert_mode, ui_normal_mode};

/// # Warning
/// Cette fonction est temporaire et sera très certainement supprimé dans le futur.
/// Crée le rendu en fonction de l'[`InputMode`](enum@InputMode) actuel.
pub fn ui(frame: &mut Frame, app: &mut App) {
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100), Constraint::Length(1)])
        .split(frame.size());

    frame.render_widget(
        Paragraph::new(
            Text::from(std::str::from_utf8(&app.text_buffer).unwrap())
        )
        .bg(Color::Rgb(0x2b, 0x2d, 0x3a))
        .scroll(app.scroll),
    area[0]);

    match app.input_mode {
        InputMode::Normal => ui_normal_mode(frame, area[1]),
        InputMode::Insert => ui_insert_mode(frame, area[1]),
        InputMode::Command => ui_command_mode(app, frame, area[1]),
        InputMode::Exit => {}
    };

    frame.render_widget(Block::default(), area[0]);
}
