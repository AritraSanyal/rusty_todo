use crate::{
    app::{App, InputMode},
    ui,
};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{backend::Backend, Terminal};

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>>
where
    B::Error: 'static,
{
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('a') => {
                        app.input_mode = InputMode::Insert;
                    }
                    KeyCode::Char('l') | KeyCode::Char('h') => app.move_focus(key.code),
                    KeyCode::Char('>') => app.move_task_forward(),
                    KeyCode::Char('<') => app.move_task_backwords(),
                    KeyCode::Char('j') => app.next_item(),
                    KeyCode::Char('k') => app.previous_item(),
                    _ => {}
                },
                InputMode::Insert => match key.code {
                    KeyCode::Enter if key.modifiers.contains(KeyModifiers::ALT) => {
                        app.submit_task();
                    }
                    KeyCode::Esc => {
                        app.input.clear();
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char(ch) => {
                        app.input.push(ch);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                },
            }
        }
    }
}
