use crate::app::{ActiveBlock, App};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, BorderType, Borders},
    Frame,
};

//fn render_helper() {
//    todo!();
//}

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

pub fn render(frame: &mut Frame, app: &App) {
    // Define the area
    let area = frame.area();

    // if the screen is big enough
    if area.width > 120 {
        // split the area into three different horizontal chunk
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
    } else if area.width > 60 {
        // split the area into main chunk and right chunk
        let main_chunks =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);

        //split the right chunk into two vertical chunks
        let right_chunk =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_chunks[1]);

        // render the chunks
        render_todo(frame, main_chunks[0], app.is_active(ActiveBlock::Todo));
        render_doing(frame, right_chunk[0], app.is_active(ActiveBlock::Doing));
        render_done(frame, right_chunk[1], app.is_active(ActiveBlock::Done));
    } else {
        let chunks = Layout::vertical([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);

        render_todo(frame, chunks[0], app.is_active(ActiveBlock::Todo));
        render_doing(frame, chunks[1], app.is_active(ActiveBlock::Doing));
        render_done(frame, chunks[2], app.is_active(ActiveBlock::Done));
    }
}
