use super::{core::Core, InputMode};

/// Réprésente la ligne de commande utilisable dans le mode [`InputMode::Command`](type@InputMode::Command)
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

/// # Warning
/// Cette fonction est temporaire et sera très certainement supprimé dans le futur
pub fn check_cmd(app: &mut Core) {
    let cmd = &app.command_line.command_buffer;
    match &cmd[..2] {
        ":q" => app.input_mode = InputMode::Exit,
        ":e" => app.editor.open_file(cmd[2..].trim().to_string()),
        _ => {}
    }
}
