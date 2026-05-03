use crossterm::event::KeyCode;
use ratatui::widgets::ListState;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ActiveBlock {
    Todo,
    Doing,
    Done,
}

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    Insert,
}

#[derive(Debug, PartialEq)]
pub struct App {
    // --- Active Block ---
    pub active_block: ActiveBlock,
    // --- Task Data ---
    pub todos: Vec<String>,
    pub doing: Vec<String>,
    pub done: Vec<String>,
    //--- State of each list ---
    pub todo_state: ListState,
    pub doing_state: ListState,
    pub done_state: ListState,
    //--- Taking Input ---
    pub input: String,
    //--- Input Mode -- between navigating and Input ---
    pub input_mode: InputMode,
}

impl App {
    pub fn new() -> Self {
        let mut todo_state = ListState::default();
        todo_state.select(Some(0));
        Self {
            active_block: ActiveBlock::Todo,
            input: String::new(),
            input_mode: InputMode::Normal,
            todos: vec![
                "Setup Life".to_string(),
                "Typping...".to_string(),
                "Die".to_string(),
            ],
            doing: vec!["Add task list".to_string()],
            done: vec![
                "Responsive Layout".to_string(),
                "Skeliton Structure".to_string(),
            ],
            todo_state,
            doing_state: ListState::default(),
            done_state: ListState::default(),
        }
    }

    // --- Helper function to submit task ---
    pub fn submit_task(&mut self) {
        if !self.input.trim().is_empty() {
            self.todos.push(self.input.trim().to_string());
        }
        self.input.clear();
        self.input_mode = InputMode::Normal;
    }

    // --- Helper Funtion to check which block is active ---
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
    // --- Helper Funtion for moving to next item on the list
    pub fn next_item(&mut self) {
        let (state, items) = match self.active_block {
            ActiveBlock::Todo => (&mut self.todo_state, &self.todos),
            ActiveBlock::Doing => (&mut self.doing_state, &self.doing),
            ActiveBlock::Done => (&mut self.done_state, &self.done),
        };

        let i = match state.selected() {
            Some(i) => {
                if i >= items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    // --- Helper function for moveing to previous item on the list ---
    pub fn previous_item(&mut self) {
        let (state, items) = match self.active_block {
            ActiveBlock::Todo => (&mut self.todo_state, &self.todos),
            ActiveBlock::Doing => (&mut self.doing_state, &self.doing),
            ActiveBlock::Done => (&mut self.done_state, &self.done),
        };

        if items.is_empty() {
            return;
        }

        let i = match state.selected() {
            Some(i) => {
                if i == 0 {
                    items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }
}
