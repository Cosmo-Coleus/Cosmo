use crate::core::{
    command_line::{text_command, CommandLine},
    queue::CommandQueue,
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
    fn cmd_check(self: &CommandLine, queue: &mut CommandQueue) {
        let parsed_cmd = self.parser();
        text_command::quit(&parsed_cmd, ":q", queue);
        text_command::edit_file(&parsed_cmd, ":e", queue);
    }
}
