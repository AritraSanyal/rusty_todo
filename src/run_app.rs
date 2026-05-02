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
            if key.kind == crossterm::event::KeyEventKind::Press {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }

                app.move_focus(key.code);
            }
        }
    }
}
