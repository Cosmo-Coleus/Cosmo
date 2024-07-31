use super::commands::Commands;

pub struct CommandQueue {
    queue: Vec<Commands>,
}

impl CommandQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn push_cmd(&mut self, cmd: Commands) {
        self.queue.push(cmd);
    }

    pub fn pop_cmd(&mut self) -> Option<Commands> {
        self.queue.pop()
    }
}
