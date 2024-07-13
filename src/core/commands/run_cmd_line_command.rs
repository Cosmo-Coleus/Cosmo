use crate::core::{command_line::CommandLine, Core};

use super::{set_mode_command::SetExitMode, write_cmd_line_command::WriteChar, Command};

pub struct RunCmdLine;

impl Command for RunCmdLine {
    fn execute_cmd_line(&mut self, cmd: &mut CommandLine) {
        cmd.check_cmd();
    }
}

impl CommandLine {
    /// Temporaire
    /// TODO: Faire une file d'attente pour gerer les commandes imbrique
    fn check_cmd(self: &mut CommandLine) {
        match self.buffer.as_str() {
            ":q" => {
                //  self.push_cmd_in_queue(WriteChar('c'));
                //  self.push_cmd_in_queue(WriteChar('c'));
                //  self.push_cmd_in_queue(WriteChar('t'));
                //  self.push_cmd_in_queue(WriteChar('o'));
                //  self.push_cmd_in_queue(WriteChar('i'));
            }
            _ => {}
        }
    }
}
