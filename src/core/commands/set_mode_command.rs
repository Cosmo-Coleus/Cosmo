use crate::core::{editor::Editor, InputMode};

use super::Command;

pub struct SetInsertMode;
pub struct SetNormalMode;
pub struct SetCommandMode;
pub struct SetExitMode;

impl Command for SetInsertMode {
    fn execute(&mut self, editor: &mut Editor) {
        editor.set_mode(InputMode::Insert);
    }
}

impl Command for SetNormalMode {
    fn execute(&mut self, editor: &mut Editor) {
        editor.set_mode(InputMode::Normal);
    }
}

impl Command for SetCommandMode {
    fn execute(&mut self, editor: &mut Editor) {
        editor.set_mode(InputMode::Command);
    }
}

impl Command for SetExitMode {
    fn execute(&mut self, editor: &mut Editor) {
        editor.set_mode(InputMode::Exit);
    }
}

impl Editor{
    fn set_mode(self: &mut Editor, mode: InputMode) {
        self.input_mode = mode;
    }
}
