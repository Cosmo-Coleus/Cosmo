use crate::core::{Core, InputMode};

use super::Command;

pub struct SetInsertMode;
pub struct SetNormalMode;
pub struct SetCommandMode;
pub struct SetExitMode;

impl Command for SetInsertMode {
    fn execute(&mut self, core: &mut Core) {
        core.set_mode(InputMode::Insert);
    }
}

impl Command for SetNormalMode {
    fn execute(&mut self, core: &mut Core) {
        core.set_mode(InputMode::Normal);
    }
}

impl Command for SetCommandMode {
    fn execute(&mut self, core: &mut Core) {
        core.set_mode(InputMode::Command);
    }
}

impl Command for SetExitMode {
    fn execute(&mut self, core: &mut Core) {
        core.set_mode(InputMode::Exit);
    }
}

impl Core{
    fn set_mode(self: &mut Core, mode: InputMode) {
        self.input_mode = mode;
    }
}
