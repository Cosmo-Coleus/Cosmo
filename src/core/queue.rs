use super::commands::{command_invoker::CommandInvoker, Command, Commands};

pub struct CommandsQueue {
    queue: Vec<Commands>,
}

impl CommandsQueue {
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
