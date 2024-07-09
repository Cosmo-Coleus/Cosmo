use std::io::Result;


use ratatui::{backend::Backend, crossterm::event::{self, Event, KeyCode, KeyEvent}, style::Stylize, widgets::Paragraph, Frame, Terminal};

#[derive(PartialEq, Eq)]
enum InputMode {
    Normal,
    Insert,
    Command,
    Exit
}

pub struct App {
    input_mode: InputMode
}

impl App {
    pub const fn new() -> Self {
        Self { input_mode: InputMode::Normal }
    }
    fn manage_input_mode(self: &mut App, key: KeyEvent)  {
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('i') => self.input_mode = InputMode::Insert,
                KeyCode::Char(':') => self.input_mode = InputMode::Command,
                KeyCode::Char('q') => self.input_mode = InputMode::Exit,
                _ => {}
            },
            InputMode::Insert => match key.code {
                KeyCode::Esc => self.input_mode = InputMode::Normal,
                _ => {}
            },
            InputMode::Command => match key.code {
                KeyCode::Esc => self.input_mode = InputMode::Normal,
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            app.manage_input_mode(key);
        }
        if app.input_mode == InputMode::Exit {
            break;
        }
    }
    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let area = frame.size();
    let text_mode = match app.input_mode {
        InputMode::Normal => "Hello NORMAL MODE",
        InputMode::Insert => "Hello INSERT MODE",
        InputMode::Command => "Hello COMMAND MODE",
        InputMode::Exit => ""
    };
    frame.render_widget(
        Paragraph::new(text_mode)
            .white()
            .on_blue(),
        area
    );
}
