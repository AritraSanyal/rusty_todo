use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, TitlePosition},
};

pub fn render(frame: &mut Frame) {
    let area = frame.area();
    let direction = if area.width < 80 {
        Direction::Vertical
    } else {
        Direction::Horizontal
    };

    let chunks = Layout::default()
        .direction(direction)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);

    let titles = ["TODOS", "DOING", "DONE"];
    for (i, title) in titles.iter().enumerate() {
        let block = Block::default()
            .title(format!(" {} ", title))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        frame.render_widget(block, chunks[i]);
    }
}
