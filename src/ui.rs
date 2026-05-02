use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, TitlePosition},
    Frame,
};

fn render_todo(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("TODOS")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(block, area);
}

fn render_doing(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("DOING")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(block, area);
}

fn render_done(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("DONE")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(block, area);
}

pub fn render(frame: &mut Frame) {
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
        render_todo(frame, chunks[0]);
        render_doing(frame, chunks[1]);
        render_done(frame, chunks[2]);
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
        render_todo(frame, main_chunks[0]);
        render_doing(frame, right_chunk[0]);
        render_done(frame, right_chunk[1]);
    } else {
        let chunks = Layout::vertical([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);

        render_todo(frame, chunks[0]);
        render_doing(frame, chunks[1]);
        render_done(frame, chunks[2]);
    }
}
