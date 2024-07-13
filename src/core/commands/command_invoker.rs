use crate::core::Core;
use super::Command;

pub struct CommandInvoker<'a> {
    pub core: &'a mut Core,
}

impl<'a> CommandInvoker<'a> {
    pub fn new(core: &'a mut Core) -> Self {
        CommandInvoker { core }
    }

    pub fn execute_command(self: &mut CommandInvoker<'a>, mut command: impl Command + 'static) {
        command.execute_core(self.core);
        command.execute_editor(&mut self.core.editor);
        command.execute_cmd_line(&mut self.core.command_line);
    }
}
