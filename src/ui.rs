use crate::app::{ActiveBlock, App};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

fn render_todo(frame: &mut Frame, area: Rect, is_active: bool) {
    let mut block = Block::default()
        .title("[TODOS]")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    if is_active {
        block = block.yellow().bold().italic();
    }
    frame.render_widget(block, area);
}

fn render_doing(frame: &mut Frame, area: Rect, is_active: bool) {
    let mut block = Block::default()
        .title("[DOING]")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    if is_active {
        block = block.yellow().bold().italic();
    }
    frame.render_widget(block, area);
}

fn render_done(frame: &mut Frame, area: Rect, is_active: bool) {
    let mut block = Block::default()
        .title("[DONE]")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    if is_active {
        block = block.yellow().bold().italic();
    }
    frame.render_widget(block, area);
}

fn render_wide_layout(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ])
    .split(area);

    // render the chunks
    render_todo(frame, chunks[0], app.is_active(ActiveBlock::Todo));
    render_doing(frame, chunks[1], app.is_active(ActiveBlock::Doing));
    render_done(frame, chunks[2], app.is_active(ActiveBlock::Done));
}

fn render_middle_layout(frame: &mut Frame, app: &App, area: Rect) {
    // split the area into main chunk and right chunk
    let main_chunks =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);

    //split the right chunk into two vertical chunks
    let right_chunk = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    // render the chunks
    render_todo(frame, main_chunks[0], app.is_active(ActiveBlock::Todo));
    render_doing(frame, right_chunk[0], app.is_active(ActiveBlock::Doing));
    render_done(frame, right_chunk[1], app.is_active(ActiveBlock::Done));
}

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
    render_todo(frame, chunks[0], app.is_active(ActiveBlock::Todo));
    render_doing(frame, chunks[1], app.is_active(ActiveBlock::Doing));
    render_done(frame, chunks[2], app.is_active(ActiveBlock::Done));
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let info_text = " Next: l | Prev: h";
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
}
