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
    // Temporaire
    fn cmd_check(self: &CommandLine, queue: &mut CommandsQueue) {
        let parsed_cmd = self.parser();
        TextCommand::q_text_command(parsed_cmd, ":q", queue)
    }
}
