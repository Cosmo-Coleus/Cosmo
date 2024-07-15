use ratatui::Frame;

pub struct Cursor {
    pub pos: (u16, u16)
}

impl Cursor {
    pub fn new(x: u16, y: u16) -> Self {
        Self { pos: (x, y) }
    }

    pub fn set_cursor(&self, frame: &mut Frame) {
        frame.set_cursor(self.pos.0, self.pos.1);
    }

    pub fn set_pos(&mut self, x: u16, y: u16) {
        self.pos.0 = x;
        self.pos.1 = y;
    }

    pub fn move_to_end_buf(&mut self, buf: &String) {
        self.pos.0 = buf.len() as u16 + 1;
    }
}
