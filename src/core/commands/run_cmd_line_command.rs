use crate::core::{command_line::CommandLine, queue::CommandsQueue, Core};

use super::{Command, Commands};

pub struct RunCmdLine;

impl Command for RunCmdLine {
fn execute_core(&mut self, core: &mut Core) {
        core.command_line.check_cmd(&mut core.queue);
    }
}

impl CommandLine {
    /// Temporaire mais c'est l'idee mdr
    fn check_cmd(self: &CommandLine, queue: &mut CommandsQueue) {
        match self.buffer.as_str() {
            ":q" => queue.push_cmd(Commands::SetExitMode),
            _ => {}
        }
    }
}
