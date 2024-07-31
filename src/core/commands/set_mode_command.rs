use super::Command;
use crate::core::{editor::Editor, Core, InputMode};

pub struct SetInsertMode;
pub struct SetNormalMode;
pub struct SetCommandMode;
pub struct SetExitMode;

impl Command for SetInsertMode {
    fn execute_editor(&mut self, editor: &mut Editor) {
        editor.set_mode(InputMode::Insert);
    }
}

impl Command for SetNormalMode {
    fn execute_editor(&mut self, editor: &mut Editor) {
        editor.set_mode(InputMode::Normal);
    }
}

impl Command for SetCommandMode {
    fn execute_core(&mut self, core: &mut Core) {
        core.editor.set_mode(InputMode::Command);
    }
}

impl Command for SetExitMode {
    fn execute_editor(&mut self, editor: &mut Editor) {
        editor.set_mode(InputMode::Exit);
    }
}

impl Editor {
    fn set_mode(self: &mut Editor, mode: InputMode) {
        self.input_mode = mode;
    }
}
