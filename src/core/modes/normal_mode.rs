use ratatui::crossterm::event::KeyCode;

use crate::core::{commands::Commands, queue::CommandsQueue};

/// Récupère toutes les touches pressées en [`InputMode::Normal`] et les associe au comportement attendu.
pub fn normal_mode(key: KeyCode, queue: &mut CommandsQueue) {
    match key {
        //KeyCode::Char('i') => invoker.execute_command(SetInsertMode),
        KeyCode::Char('q') => {
            //invoker.execute_command(SetExitMode)
            queue.push_cmd(Commands::SetExitMode)
        },
        //KeyCode::Char(':') => {
        //    invoker.execute_command(SetCommandMode);
//invoker.execute_command(WriteChar(':'));
       // }
       // KeyCode::Down => invoker.execute_command(ScrollUpCommand),
        //KeyCode::Up => invoker.execute_command(ScrollDownCommand),
        _ => {}
    }
}
