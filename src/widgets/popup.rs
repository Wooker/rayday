use std::ops::Div;

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::InputMode;

pub const WIDTH: u16 = 30;
pub const HEIGHT: u16 = 8;

pub struct PopupAdd<'a> {
    time: &'a String,
    description: &'a String,
    input_mode: &'a InputMode,
    pub block: Option<Block<'a>>,
    error_message: Option<&'a str>,
}

impl<'a> PopupAdd<'a> {
    pub fn new(time: &'a String, description: &'a String, input_mode: &'a InputMode) -> Self {
        PopupAdd {
            time,
            description,
            input_mode,
            block: None,
            error_message: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> PopupAdd<'a> {
        self.block = Some(block);
        self
    }
}

pub fn centered_rect(height: u16, width: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(r.height / 2 - height / 2),
                Constraint::Length(height),
                Constraint::Length(r.height / 2 - height / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(r.width / 2 - width / 2),
                Constraint::Length(width),
                Constraint::Length(r.width / 2 - width / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

impl<'a> Widget for PopupAdd<'a> {
    fn render(mut self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let title = String::from("Time");
        let block_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let layout = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .direction(Direction::Vertical)
            .split(block_area);

        let time_par = Paragraph::new(self.time.as_ref())
            .style(match self.input_mode {
                InputMode::AddingTime => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("Time (hh:mm)"));
        time_par.render(layout[0], buf);

        let description_par = Paragraph::new(self.description.as_ref())
            .style(match self.input_mode {
                InputMode::AddingDescription => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("Description"));
        description_par.render(layout[1], buf);
    }
}
