mod ui_editor_modes;
mod ui_editor_view;

use crate::core::{Core, InputMode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Block,
    Frame,
};
use ui_editor_modes::{ui_command_mode, ui_insert_mode, ui_normal_mode};
use ui_editor_view::ui_editor_view;

/// # Warning
/// Cette fonction est temporaire et sera très certainement supprimé dans le futur.
/// Crée le rendu en fonction de l'[`InputMode`](enum@InputMode) actuel.
pub fn ui(frame: &mut Frame, app: &mut Core) {
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100), Constraint::Length(1)])
        .split(frame.size());

    frame.render_widget(Block::default(), area[0]);
    ui_editor_view(app, frame, area[0]);

    match app.editor.input_mode {
        InputMode::Normal => ui_normal_mode(frame, area[1]),
        InputMode::Insert => ui_insert_mode(frame, area[1]),
        InputMode::Command => ui_command_mode(app, frame, area[1]),
        InputMode::Exit => {}
    };
}
