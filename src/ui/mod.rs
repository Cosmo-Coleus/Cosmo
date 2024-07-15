mod editor;
mod modes;
mod command_line;

use crate::core::{Core, InputMode};
use modes::{command_mode, insert_mode, normal_mode};
use ratatui::{
    layout::{Constraint, Direction, Layout}, style::{Color, Stylize}, widgets::Block, Frame
};

pub fn draw(frame: &mut Frame, core: &mut Core) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100), Constraint::Length(1)])
        .split(frame.size());

    let editor_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Length(5), Constraint::Percentage(100)])
        .split(chunks[0]);

    let cmd_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(100), Constraint::Length(14)])
        .split(chunks[1]);

    //frame.render_widget(Block::default().bg(Color::Blue), editor_layout[0]); // Number
    //frame.render_widget(Block::default().bg(Color::Red), editor_layout[1]); // Editor
    //frame.render_widget(Block::default().bg(Color::Green), cmd_layout[0]); // CMD
    //frame.render_widget(Block::default().bg(Color::LightBlue), cmd_layout[1]); // CMD

    editor::draw(frame, core, chunks[0]);

    match core.editor.input_mode {
        InputMode::Normal => normal_mode::draw(core, frame, cmd_layout),
        InputMode::Insert => insert_mode::draw(core, frame, cmd_layout),
        InputMode::Command => command_mode::draw(core, frame, cmd_layout),
        InputMode::Exit => {}
    };
    core.set_cursor(frame);
}
