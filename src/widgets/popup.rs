use tui::{
    layout::{Layout, Direction, Constraint, Rect},
    widgets::{Widget, Paragraph, Borders, Block},
    style::{Style, Color},
};

use crate::app::InputMode;

pub struct PopupAdd<'a> {
    input: &'a String,
    input_mode: &'a InputMode,
    block: Option<Block<'a>>,
}

impl<'a> PopupAdd<'a> {
    pub fn new(input: &'a String, input_mode: &'a InputMode) -> Self {
        PopupAdd { input, input_mode, block: None }
    }

    pub fn block(mut self, block: Block<'a>) -> PopupAdd<'a> {
        self.block = Some(block);
        self
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

impl<'a> Widget for PopupAdd<'a> {
    fn render(mut self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let block_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            },
            None => area,
        };

        let input = Paragraph::new(self.input.as_ref())
            .style(match self.input_mode {
                InputMode::Adding => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("Add Event"));
        input.render(area, buf);
    }
}
