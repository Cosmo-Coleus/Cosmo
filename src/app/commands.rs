use super::{App, InputMode};

pub struct CommandLine {
    pub command_buffer: String,
}

impl CommandLine {
    pub const fn new() -> Self {
        Self {
            command_buffer: String::new(),
        }
    }
}

pub fn check_cmd(app: &mut App) {
    let cmd = &app.command_line.command_buffer;
    if cmd == ":q" {
        app.input_mode = InputMode::Exit;
    }
}
