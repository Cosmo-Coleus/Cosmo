use crate::core::Core;

use super::Command;

pub struct CommandInvoker<'a> {
    core: &'a mut Core
}

impl<'a> CommandInvoker<'a> {
    pub fn new(core: &'a mut Core) -> Self {
        CommandInvoker { core }
    }

    pub fn execute_command(self: &mut CommandInvoker<'a>, mut command: impl Command + 'static) {
        command.execute(self.core);
    }
}
