mod ui_editor_modes;
use crate::app::{App, InputMode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Block,
    Frame,
};
use ui_editor_modes::{ui_command_mode, ui_insert_mode, ui_normal_mode};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100), Constraint::Length(1)])
        .split(frame.size());

    match app.input_mode {
        InputMode::Normal => ui_normal_mode(frame, area[1]),
        InputMode::Insert => ui_insert_mode(frame, area[1]),
        InputMode::Command => ui_command_mode(app, frame, area[1]),
        InputMode::Exit => {}
    };

    frame.render_widget(Block::default(), area[0]);
}
