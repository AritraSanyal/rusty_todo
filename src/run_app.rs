use crate::{app::App, ui};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>>
where
    B::Error: 'static,
{
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('l') | KeyCode::Char('h') => app.move_focus(key.code),
                KeyCode::Char('j') => app.next_item(),
                KeyCode::Char('k') => app.previous_item(),
                _ => {}
            }
        }
    }
}
