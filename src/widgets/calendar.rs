use num_traits::cast::FromPrimitive;
use chrono::{Date, Local, Datelike, Weekday, Month, Duration};
use tui::{
    buffer::Buffer,
    layout::{Corner, Rect},
    style::{Style, Color},
    text::{Text, Spans, Span},
    widgets::{Block, StatefulWidget, Widget},
};
use unicode_width::UnicodeWidthStr;

use crate::{
    app::InputMode,
    widgets::weeks::Weeks
};

const DAY_WIDTH: u8 = 2;


#[derive(Debug)]
pub struct CalendarWidget<'a> {
    selected: Date<Local>,
    today: Date<Local>,
    style: Style,
    block: Option<Block<'a>>,
    highlight_style: Style,
    highlight_symbol: Option<&'a str>,
    //days: Vec<Date<Local>>,
    content: Text<'a>
}

impl<'a> CalendarWidget<'a> {
    pub fn new(weeks: Weeks<'a>, selected: Date<Local>, input_mode: &InputMode) -> Self {
        let today = Local::now().date();

        CalendarWidget {
            selected,
            today,
            style: Style::default(),
            block: None,
            highlight_style: Style::default(),
            highlight_symbol: None,
            //days,
            content: weeks.content()
        }
    }

    pub fn get_date(&self) -> Date<Local> {
        self.selected
    }

    pub fn block(mut self, block: Block<'a>) -> CalendarWidget<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> CalendarWidget<'a> {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> CalendarWidget<'a> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> CalendarWidget<'a> {
        self.highlight_style = style;
        self
    }
}

impl<'a> StatefulWidget for CalendarWidget<'a> {
    type State = Date<Local>;

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

        //println!("{}", self.content.lines.len());
        for spans in self.content.into_iter() {
            buf.set_spans(x, y, &spans, block_area.width);
            y += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::Calendar;

}
