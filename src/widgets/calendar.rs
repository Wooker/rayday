use chrono::{Datelike, Duration, Local, Month, NaiveDate, Weekday};
use crossterm::style::Colored;
use num_traits::cast::FromPrimitive;
use tui::{
    buffer::Buffer,
    layout::{Corner, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, StatefulWidget, Widget},
};
use unicode_width::UnicodeWidthStr;

use crate::{app::InputMode, widgets::weeks::Weeks};

const DAY_WIDTH: u8 = 2;

#[derive(Debug)]
pub struct CalendarState {
    selected_date: NaiveDate,
}

impl CalendarState {
    pub fn new(selected_date: NaiveDate) -> Self {
        Self { selected_date }
    }
    pub fn get_selected_date(&self) -> NaiveDate {
        self.selected_date
    }
}

#[derive(Debug)]
pub struct CalendarWidget<'a> {
    today: NaiveDate,
    style: Style,
    block: Option<Block<'a>>,
    highlight_style: Style,
    highlight_symbol: Option<&'a str>,
    content: Text<'a>,
}

impl<'a> CalendarWidget<'a> {
    pub fn new() -> Self {
        let today = Local::now().date_naive();

        CalendarWidget {
            today,
            style: Style::default(),
            block: None,
            highlight_style: Style::default(),
            highlight_symbol: None,
            content: Text::raw(""),
        }
    }

    pub fn with_weeks(mut self, weeks: Weeks<'a>) -> Self {
        self.content = weeks.content();
        self
    }

    pub fn block(mut self, block: Block<'a>) -> CalendarWidget<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, input_mode: &InputMode) -> CalendarWidget<'a> {
        self.style = match input_mode {
            InputMode::Normal => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        };
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> CalendarWidget<'a> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, highlight_color: Color) -> CalendarWidget<'a> {
        self.highlight_style = Style::default()
            .bg(highlight_color)
            .add_modifier(Modifier::BOLD);
        self
    }
}

impl<'a> Default for CalendarWidget<'a> {
    fn default() -> Self {
        CalendarWidget::new().block(
            Block::default()
                .borders(Borders::ALL)
                .title("Calendar Widget"),
        )
    }
}

impl<'a> StatefulWidget for CalendarWidget<'a> {
    type State = CalendarState; //Date<Local>;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);

        let block_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let (mut x, mut y) = (block_area.left(), block_area.top());

        for spans in self.content.into_iter() {
            buf.set_spans(x, y, &spans, block_area.width);
            y += 1;
        }
    }
}
