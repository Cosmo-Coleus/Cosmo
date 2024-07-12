use super::editor::Editor;

pub mod command_invoker;
pub mod scroll_command;
pub mod set_mode_command;

pub trait Command {
    fn execute(&mut self, core: &mut Editor);
}
