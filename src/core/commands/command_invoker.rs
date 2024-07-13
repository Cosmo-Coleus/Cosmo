use crate::core::{queue::CommandsQueue, Core};
use super::{set_mode_command::SetExitMode, Command, Commands};

pub struct CommandInvoker<'a> {
    pub core: &'a mut Core,
}

impl<'a> CommandInvoker<'a> {
    pub fn new(core: &'a mut Core) -> Self {
        CommandInvoker { core }
    }
    pub fn run_cmd(&mut self) {
        if let Some(cmd) = self.core.queue.pop_cmd() {
            self.execute_command(cmd);
        }
    }

    pub fn execute_command(self: &mut CommandInvoker<'a>, cmds: Commands) {
        //command.execute_core(self.core);
        let mut cmd = match cmds {
            Commands::ScrollUp => todo!(),
            Commands::ScrollDown => todo!(),
            Commands::SetInsertMode => todo!(),
            Commands::SetNormalMode => todo!(),
            Commands::SetCommandMode => todo!(),
            Commands::SetExitMode => SetExitMode,
            Commands::WriteChar => todo!(),
            Commands::RemoveChar => todo!(),
            Commands::CleanBuffer => todo!(),
            Commands::RunCmdLine => todo!(),
        };
        cmd.execute_editor(&mut self.core.editor);
        //command.execute_cmd_line(&mut self.core.command_line);
    }
}
