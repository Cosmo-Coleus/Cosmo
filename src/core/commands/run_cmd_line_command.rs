use crate::core::{
    command_line::{CommandLine, TextCommand},
    queue::CommandsQueue,
    Core,
};

use super::Command;

pub struct RunCmdLine;

impl Command for RunCmdLine {
    fn execute_core(&mut self, core: &mut Core) {
        core.command_line.cmd_check(&mut core.queue);
    }
}

impl CommandLine {
    fn cmd_check(self: &CommandLine, queue: &mut CommandsQueue) {
        let parsed_cmd = self.parser();
        TextCommand::quit_text_command(&parsed_cmd, ":q", queue);
        TextCommand::edit_file_text_command(&parsed_cmd, ":e", queue);
    }
}
