use super::{
    manage_file::OpenFile,
    run_cmd_line_command::RunCmdLine,
    scroll_command::{ResetScroll, ScrollDown, ScrollUp},
    set_mode_command::{SetCommandMode, SetExitMode, SetInsertMode, SetNormalMode},
    write_cmd_line_command::{CleanBuffer, RemoveChar, WriteChar},
    Command, Commands,
};
use crate::core::Core;

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
        let mut cmd: Box<dyn Command> = match cmds {
            Commands::ResetScroll => Box::new(ResetScroll),
            Commands::ScrollUp => Box::new(ScrollUp),
            Commands::ScrollDown => Box::new(ScrollDown),
            Commands::SetInsertMode => Box::new(SetInsertMode),
            Commands::SetNormalMode => Box::new(SetNormalMode),
            Commands::SetCommandMode => Box::new(SetCommandMode),
            Commands::SetExitMode => Box::new(SetExitMode),
            Commands::WriteChar(ch) => Box::new(WriteChar(ch)),
            Commands::RemoveChar => Box::new(RemoveChar),
            Commands::CleanBuffer => Box::new(CleanBuffer),
            Commands::RunCmdLine => Box::new(RunCmdLine),
            Commands::OpenFile(file) => Box::new(OpenFile(file)),
        };
        cmd.execute_core(self.core);
        cmd.execute_editor(&mut self.core.editor);
        cmd.execute_cmd_line(&mut self.core.command_line);
    }
}
