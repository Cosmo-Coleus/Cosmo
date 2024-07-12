use crate::core::editor::Editor;

use super::Command;

pub struct CommandInvoker<'a> {
    pub editor: &'a mut Editor
}

impl<'a> CommandInvoker<'a> {
    pub fn new(editor: &'a mut Editor) -> Self {
        CommandInvoker { editor }
    }

    pub fn execute_command(self: &mut CommandInvoker<'a>, mut command: impl Command + 'static) {
        command.execute(self.editor);
    }
}
