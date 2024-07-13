use super::commands::{command_invoker::CommandInvoker, Command};

pub struct CommandsQueue {
    queue: Vec<Box<dyn Command>>
}

impl CommandsQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn push_cmd_in_queue<T: Command + 'static>(&mut self, cmd: T) {
        self.queue.push(Box::new(cmd));
    }

    pub fn pop_cmd_in_queue(&mut self) {
        self.queue.pop();
    }

    pub fn run_cmd_in_queue(&mut self, invoker: &mut CommandInvoker) {
        match self.queue.first_mut() {
            Some(cmd) => {
                //invoker.execute_command(cmd.as_mut())
            },
            None => {}
        }
        //self.pop_cmd_in_queue();
    }
}
