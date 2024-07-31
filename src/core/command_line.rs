pub struct ParsedCommand {
    pub cmd: String,
    pub args: Vec<String>,
}

/// Représente la CLI utilisable dans le mode [`InputMode::Command`](type@InputMode::Command)
pub struct CommandLine {
    pub buffer: String,
}

impl CommandLine {
    pub const fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn parser(&self) -> ParsedCommand {
        let buf = self.buffer.clone();
        let mut tokens: Vec<String> = buf.split(' ').map(str::to_string).collect();
        ParsedCommand {
            cmd: tokens[0].clone(),
            args: tokens.split_off(1),
        }
    }
}

pub mod text_command {
    use super::ParsedCommand;
    use crate::core::{commands::Commands, queue::CommandQueue};

    pub fn quit(parsed_command: &ParsedCommand, cmd: &str, queue: &mut CommandQueue) {
        if parsed_command.cmd != cmd {
            return;
        }
        if !parsed_command.args.is_empty() {
            return;
        }
        queue.push_cmd(Commands::SetExitMode);
    }

    pub fn edit_file(parsed_command: &ParsedCommand, cmd: &str, queue: &mut CommandQueue) {
        if parsed_command.cmd != cmd {
            return;
        }
        if parsed_command.args.is_empty() || parsed_command.args.len() != 1 {
            return;
        }
        queue.push_cmd(Commands::OpenFile(parsed_command.args[0].clone()));
    }
}
