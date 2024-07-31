use super::Command;
use crate::core::command_line::CommandLine;

pub struct WriteChar(pub char);
pub struct RemoveChar;
pub struct CleanBuffer;

impl Command for WriteChar {
    fn execute_cmd_line(&mut self, cmd: &mut CommandLine) {
        cmd.add_char(self.0);
    }
}

impl Command for RemoveChar {
    fn execute_cmd_line(&mut self, cmd: &mut CommandLine) {
        cmd.remove_char();
    }
}

impl Command for CleanBuffer {
    fn execute_cmd_line(&mut self, cmd: &mut CommandLine) {
        cmd.clean_buffer();
    }
}

impl CommandLine {
    fn add_char(self: &mut CommandLine, ch: char) {
        self.buffer.push(ch);
    }

    fn remove_char(self: &mut CommandLine) {
        self.buffer.pop();
    }

    fn clean_buffer(self: &mut CommandLine) {
        self.buffer.clear();
    }
}
