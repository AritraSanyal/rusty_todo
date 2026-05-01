use crate::ui;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{Terminal, backend::Backend};

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>>
where
    B::Error: 'static,
{
    loop {
        // This is where the magic happens:
        // It passes the 'Frame' from the terminal to your ui.rs function
        terminal.draw(|f| ui::render(f))?;

        // Basic event handling to let you exit
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}
