use super::{
    commands::Commands, queue::CommandsQueue
};

pub struct ParsedCommand {
    pub cmd: String,
    pub args: Vec<String>,
}

/// Réprésente la ligne de commande utilisable dans le mode [`InputMode::Command`](type@InputMode::Command)
pub struct CommandLine {
    pub buffer: String,
}

impl CommandLine {
    pub fn new() -> Self {
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

pub struct TextCommand;

impl TextCommand {
    pub fn quit_text_command(parsed_command: &ParsedCommand, cmd: &str, queue: &mut CommandsQueue) {
        if parsed_command.cmd != cmd {
            return;
        }
        if !parsed_command.args.is_empty() {
            return;
        }
        queue.push_cmd(Commands::SetExitMode);
    }

    pub fn edit_file_text_command(parsed_command: &ParsedCommand, cmd: &str, queue: &mut CommandsQueue) {
        if parsed_command.cmd != cmd {
            return;
        }
        if parsed_command.args.is_empty() || parsed_command.args.len() != 1 {
            return;
        }
        queue.push_cmd(Commands::OpenFile(parsed_command.args[0].clone()));
    }
}
