use crossterm::event::KeyCode;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ActiveBlock {
    Todo,
    Doing,
    Done,
}

#[derive(Debug, PartialEq)]
pub struct App {
    pub active_block: ActiveBlock,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_block: ActiveBlock::Todo,
        }
    }

    pub fn is_active(&self, block: ActiveBlock) -> bool {
        self.active_block == block
    }

    pub fn move_focus(&mut self, key: KeyCode) {
        match (self.active_block, key) {
            // --- MOVING RIGHT or LEFT ---
            (ActiveBlock::Todo, KeyCode::Char('l')) => {
                self.active_block = ActiveBlock::Doing;
            }
            (ActiveBlock::Todo, KeyCode::Char('h')) => {
                self.active_block = ActiveBlock::Done;
            }
            (ActiveBlock::Doing, KeyCode::Char('l')) => {
                self.active_block = ActiveBlock::Done;
            }
            (ActiveBlock::Doing, KeyCode::Char('h')) => {
                self.active_block = ActiveBlock::Todo;
            }
            (ActiveBlock::Done, KeyCode::Char('l')) => {
                self.active_block = ActiveBlock::Todo;
            }
            (ActiveBlock::Done, KeyCode::Char('h')) => {
                self.active_block = ActiveBlock::Doing;
            }

            _ => {}
        }
    }
}
