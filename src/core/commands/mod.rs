use super::{command_line::CommandLine, editor::Editor, Core};

pub mod command_invoker;
pub mod scroll_command;
pub mod set_mode_command;
pub mod write_cmd_line_command;

pub trait Command {
    fn execute_core(&mut self, _core: &mut Core) {}
    fn execute_editor(&mut self, _editor: &mut Editor) {}
    fn execute_cmd_line(&mut self, _cmd: &mut CommandLine) {}
}
