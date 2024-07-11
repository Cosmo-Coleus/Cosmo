use std::io::BufRead;

pub struct Editor {
    pub scroll: (u16, u16),
    pub text_buffer: Vec<u8>,
}

impl Editor {
    pub fn new() -> Self {
        let text = "alex
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
copucou
papa
"
        .as_bytes();
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(text);
        Self {
            scroll: (0, 0),
            text_buffer: buffer,
        }
    }

    /// Permet de scroller le text de [`Editor`](struct@Editor) d'un ligne vers le haut
    pub fn scroll_up(self: &mut Editor) {
        let num_lines = self.text_buffer.lines().count();
        if self.scroll.0 < num_lines as u16 - 1 {
            self.scroll.0 += 1;
        }
    }

    /// Permet de scroller le text de [`Editor`](struct@Editor) d'un ligne vers le bas
    pub fn scroll_down(self: &mut Editor) {
        if self.scroll.0 > 0 {
            self.scroll.0 -= 1;
        }
    }
}
