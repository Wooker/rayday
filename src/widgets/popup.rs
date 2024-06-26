use std::ops::Div;

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::{
    app::InputMode,
    popup::{input::PopupInput, state::PopupState},
};

pub const WIDTH: u16 = 60;
pub const HEIGHT: u16 = 16;

pub struct PopupWidget<'a> {
    input: PopupInput,
    // input_mode: &'a InputMode,
    pub block: Option<Block<'a>>,
    error_message: Option<&'a str>,
}

impl<'a> PopupWidget<'a> {
    pub fn new() -> Self {
        let input = PopupInput::default();
        PopupWidget {
            input,
            // input_mode,
            block: None,
            error_message: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> PopupWidget<'a> {
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

impl<'a> StatefulWidget for PopupWidget<'a> {
    type State = PopupState;

    fn render(mut self, area: Rect, buf: &mut tui::buffer::Buffer, state: &mut Self::State) {
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
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ]
                .as_ref(),
            )
            .direction(Direction::Vertical)
            .split(block_area);

        let start_layout = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .direction(Direction::Horizontal)
            .split(layout[0]);

        let end_layout = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .direction(Direction::Horizontal)
            .split(layout[1]);

        // Start form fields
        let start_date_par = Paragraph::new(self.input.start_time.as_ref())
            // .style(match state.mode {
            //     InputMode::AddingTime => Style::default().fg(Color::Yellow),
            //     _ => Style::default(),
            // })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Start date (YYYY-MM-DD)"),
            );
        start_date_par.render(start_layout[0], buf);
        let start_time_par = Paragraph::new(self.input.start_time.as_ref())
            // .style(match state.mode {
            //     InputMode::AddingTime => Style::default().fg(Color::Yellow),
            //     _ => Style::default(),
            // })
            .block(Block::default().borders(Borders::ALL).title("Time (hh:mm)"));
        start_time_par.render(start_layout[1], buf);

        // End form fields
        let end_date_par = Paragraph::new(self.input.end_time.as_ref())
            // .style(match state.mode {
            //     InputMode::AddingTime => Style::default().fg(Color::Yellow),
            //     _ => Style::default(),
            // })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Start date (YYYY-MM-DD)"),
            );
        end_date_par.render(end_layout[0], buf);
        let end_time_par = Paragraph::new(self.input.end_time.as_ref())
            // .style(match state.mode {
            //     InputMode::AddingTime => Style::default().fg(Color::Yellow),
            //     _ => Style::default(),
            // })
            .block(Block::default().borders(Borders::ALL).title("Time (hh:mm)"));
        end_time_par.render(end_layout[1], buf);

        let description_par = Paragraph::new(self.input.description.as_ref())
            // .style(match state.mode {
            //     InputMode::AddingDescription => Style::default().fg(Color::Yellow),
            //     _ => Style::default(),
            // })
            .block(Block::default().borders(Borders::ALL).title("Description"));
        description_par.render(layout[2], buf);
    }
}
