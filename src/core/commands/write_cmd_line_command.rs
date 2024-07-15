use super::Command;
use crate::core::command_line::CommandLine;

pub struct WriteChar(pub char);
pub struct RemoveChar;
pub struct CleanBuffer;

impl Command for WriteChar {
    fn execute_cmd_line(&mut self, cmd: &mut CommandLine) {
        cmd.cursor.move_to_end_buf(&cmd.buffer);
        cmd.add_char_in_cmd_line(self.0);
    }
}

impl Command for RemoveChar {
    fn execute_cmd_line(&mut self, cmd: &mut CommandLine) {
        cmd.remove_char_in_cmd_line();
    }
}

impl Command for CleanBuffer {
    fn execute_cmd_line(&mut self, cmd: &mut CommandLine) {
        cmd.clean_buffer();
    }
}

impl CommandLine {
    fn add_char_in_cmd_line(self: &mut CommandLine, ch: char) {
        self.buffer.push(ch);
    }

    fn remove_char_in_cmd_line(self: &mut CommandLine) {
        self.buffer.pop();
    }

    fn clean_buffer(self: &mut CommandLine) {
        self.buffer.clear();
    }
}
