use core::Core;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};
use std::io::{stdout, Result};

/// Gestion de l'ensemble de **Cosmo**
mod core;
/// Gestion des inputs de **Cosmo**
mod input;
mod ui;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    init_and_run()?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

/// Initialise [`Terminal`](struct@Terminal) et [`App`](struct@App) et run le tout.
fn init_and_run() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut core = Core::new();
    terminal.clear()?;
    core.run_app(&mut terminal)?;
    terminal.show_cursor()?;
    Ok(())
}
