use super::Command;
use crate::core::editor::Editor;
use std::io::BufRead;

pub struct ScrollUp;
pub struct ScrollDown;

impl Command for ScrollUp {
    fn execute_editor(&mut self, editor: &mut Editor) {
        editor.scroll_up();
    }
}

impl Command for ScrollDown {
    fn execute_editor(&mut self, editor: &mut Editor) {
        editor.scroll_down();
    }
}

impl Editor {
    /// Permet de scroller le text de [`Editor`](struct@Editor) d'un ligne vers le haut
    fn scroll_up(self: &mut Editor) {
        let num_lines = self.text_buffer.lines().count();
        if num_lines == 0 {
            return;
        }
        if self.scroll.0 < num_lines as u16 - 1 {
            self.scroll.0 += 1;
        }
    }

    /// Permet de scroller le text de [`Editor`](struct@Editor) d'un ligne vers le bas
    fn scroll_down(self: &mut Editor) {
        if self.scroll.0 > 0 {
            self.scroll.0 -= 1;
        }
    }
}