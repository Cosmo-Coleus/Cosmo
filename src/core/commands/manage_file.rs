use crate::core::editor::Editor;
use super::Command;

pub struct OpenFile(pub String);

impl Command for OpenFile {
    fn execute_editor(&mut self, editor: &mut Editor) {
        editor.reset_scroll();
        editor.open_file(&self.0);
    }
}
