use crate::app::{ActiveBlock, App, InputMode};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

// --- helper function to create centered rect for popup
fn center_rect(percentage_x: u16, percentage_y: u16, rect: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percentage_y) / 2), // Top Margin
        Constraint::Percentage(percentage_y),             // Popup height
        Constraint::Percentage((100 - percentage_y) / 2), // Bottom Margin
    ])
    .split(rect);
    Layout::horizontal([
        Constraint::Percentage((100 - percentage_x) / 2), // Left Margin
        Constraint::Percentage(percentage_x),             // Popup Width
        Constraint::Percentage((100 - percentage_x) / 2), // Right Margin
    ])
    .split(popup_layout[1])[1]
}

// --- function to render add task popup ---
fn render_add_task_popup(frame: &mut Frame, app: &App) {
    // Create centered box
    let area = center_rect(60, 10, frame.area());

    // clear background
    frame.render_widget(Clear, area);

    // build input box
    let input_block = Block::default()
        .title_top("[Add Tasks]")
        .title_bottom(Line::from(" <a-enter> to save, <esc> to cancel").right_aligned())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().bold().cyan());

    // render the text
    let input_text = Paragraph::new(app.input.as_str()).block(input_block);
    frame.render_widget(input_text, area);

    // move terminal cursor
    frame.set_cursor_position((area.x + app.input.len() as u16 + 1, area.y + 1));
}

// --- function to render todo ---
fn render_todo(frame: &mut Frame, area: Rect, app: &App) {
    let is_active = app.is_active(ActiveBlock::Todo);

    let items: Vec<ListItem> = app
        .todos
        .iter()
        .map(|i| ListItem::new(format!("[  ] {}", i)))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title_top("[TODOS]")
                .title_bottom(Line::from(format!("{}", app.todos.len())).right_aligned())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(if is_active {
                    Style::new().bold().italic()
                } else {
                    Style::new().dark_gray()
                }),
        )
        .highlight_symbol(if is_active { " - " } else { "" })
        .highlight_style(if is_active {
            Style::new().bold().cyan()
        } else {
            Style::new()
        });
    frame.render_stateful_widget(list, area, &mut app.todo_state.clone());
}

// --- funtion to render doing section ---
fn render_doing(frame: &mut Frame, area: Rect, app: &App) {
    let is_active = app.is_active(ActiveBlock::Doing);

    let items: Vec<ListItem> = app
        .doing
        .iter()
        .map(|i| ListItem::new(format!("[!] {}", i)))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("[DOING]")
                .title_bottom(Line::from(format!("{}", app.doing.len())).right_aligned())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(if is_active {
                    Style::new().bold().italic()
                } else {
                    Style::new().dark_gray()
                }),
        )
        .highlight_symbol(if is_active { " - " } else { "" })
        .highlight_style(if is_active {
            Style::new().bold().yellow()
        } else {
            Style::new()
        });
    frame.render_stateful_widget(list, area, &mut app.doing_state.clone());
}

// --- function to render done section ---
fn render_done(frame: &mut Frame, area: Rect, app: &App) {
    let is_active = app.is_active(ActiveBlock::Done);

    let items: Vec<ListItem> = app
        .done
        .iter()
        .map(|i| ListItem::new(format!("[x] {}", i)))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("[DONE]")
                .title_bottom(Line::from(format!("{}", app.done.len())).right_aligned())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(if is_active {
                    Style::new().bold().italic()
                } else {
                    Style::new().dark_gray()
                }),
        )
        .highlight_symbol(if is_active { " - " } else { "" })
        .highlight_style(if is_active {
            Style::new().bold().green()
        } else {
            Style::new()
        });
    frame.render_stateful_widget(list, area, &mut app.done_state.clone());
}

// --- render wide layout (todo|doing|done) ---
fn render_wide_layout(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ])
    .split(area);

    // render the chunks
    render_todo(frame, chunks[0], app);
    render_doing(frame, chunks[1], app);
    render_done(frame, chunks[2], app);
}

// --- render middle layout (todo| (doing//done)) ---
fn render_middle_layout(frame: &mut Frame, app: &App, area: Rect) {
    // split the area into main chunk and right chunk
    let main_chunks =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);

    //split the right chunk into two vertical chunks
    let right_chunk = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    // render the chunks
    render_todo(frame, main_chunks[0], app);
    render_doing(frame, right_chunk[0], app);
    render_done(frame, right_chunk[1], app);
}

// --- render narrow layout (todo // doing // done) ---
fn render_narrow_layout(frame: &mut Frame, app: &App, area: Rect) {
    // We use a match to determine which block gets the "Fill"
    let constraints = match app.active_block {
        ActiveBlock::Todo => [
            Constraint::Fill(1),   // Give TODOS all the space
            Constraint::Length(3), // Force DOING to 3 lines
            Constraint::Length(3), // Force DONE to 3 lines
        ],
        ActiveBlock::Doing => [
            Constraint::Length(3), // Force TODOS to 3 lines
            Constraint::Fill(1),   // Give DOING all the space
            Constraint::Length(3), // Force DONE to 3 lines
        ],
        ActiveBlock::Done => [
            Constraint::Length(3), // Force TODOS to 3 lines
            Constraint::Length(3), // Force DOING to 3 lines
            Constraint::Fill(1),   // Give DONE all the space
        ],
    };

    // Apply the split to the vertical area
    let chunks = Layout::vertical(constraints).split(area);

    // Pass the chunks to your helpers
    render_todo(frame, chunks[0], app);
    render_doing(frame, chunks[1], app);
    render_done(frame, chunks[2], app);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let info_text = " Next Section: l | Prev Section: h | Next Item: j | Prev Item: k";
    let footer = Paragraph::new(info_text)
        .style(Style::new().dark_gray())
        .alignment(Alignment::Left);
    frame.render_widget(footer, area);
}

pub fn render(frame: &mut Frame, app: &App) {
    // Define the area
    let area = frame.area();

    let root_chunks = Layout::vertical([Constraint::Min(3), Constraint::Length(1)]).split(area);

    let main_area = root_chunks[0];
    let footer_area = root_chunks[1];

    // if the screen is big enough
    if area.width > 120 {
        render_wide_layout(frame, app, main_area);
    } else if area.width > 60 {
        render_middle_layout(frame, app, main_area);
    } else {
        render_narrow_layout(frame, app, main_area);
    }
    render_footer(frame, footer_area);

    if app.input_mode == InputMode::Insert {
        render_add_task_popup(frame, app);
    }
}
