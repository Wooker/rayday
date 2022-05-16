use num_traits::cast::FromPrimitive;
use chrono::{Date, Local, Datelike, Weekday, Month};
use tui::{
    buffer::Buffer,
    layout::{Corner, Rect},
    style::{Style, Color},
    text::{Text, Spans, Span},
    widgets::{Block, StatefulWidget, Widget},
};
use unicode_width::UnicodeWidthStr;

use crate::app::InputMode;

const DAY_WIDTH: u8 = 2;

#[derive(Debug, Clone)]
pub struct DayWidget<'a> {
    content: Text<'a>,
    style: Style,
    date: Date<Local>,
}

impl<'a> DayWidget<'a> {
    fn new(date: Date<Local>) -> Self {
        DayWidget {
            content: date.day().to_string().into(),
            style: Style::default(),
            date,
        }
    }

    fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

#[derive(Debug)]
pub struct MonthWidget<'a> {
    name: &'a str,
    style: Style,
    days: Vec<DayWidget<'a>>,
    height: usize,
}

impl<'a> MonthWidget<'a> {
    fn new(month_num: u32) -> Self {
        MonthWidget {
            days: Vec::new(),
            name: Month::from_u32(month_num).unwrap().name(),
            style: Style::default(),
            height: 1,
        }
    }

    fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

#[derive(Debug)]
pub struct CalendarWidget<'a> {
    months: Vec<MonthWidget<'a>>,
    selected: Date<Local>,
    today: Date<Local>,
    style: Style,
    block: Option<Block<'a>>,
    highlight_style: Style,
    highlight_symbol: Option<&'a str>,

    first_date: Date<Local>,
    last_date: Date<Local>,
}

impl<'a> CalendarWidget<'a> {
    pub fn new(first_date: Date<Local>, height: u16, selected: Date<Local>, input_mode: &InputMode) -> Self {
        let mut months: Vec<MonthWidget> = Vec::new();

        let mut curr_date = first_date;

        // i, j - new indicies for (month, day)
        let mut curr_month = MonthWidget::new(curr_date.month()).style(Style::default().fg(Color::White));

        let mut h = 0;
        while h < height - 2 {
            // When new month starts store the month in `months` vec and clear `month`
            if curr_date.day() == 1 && !curr_month.days.is_empty() {
                //let mut height = curr_month.days.len() / 7 + 1; // days + slot for month name
                if curr_date.weekday() != Weekday::Mon {
                    curr_month.height += 1;
                }
                months.push(curr_month);
                curr_month = MonthWidget::new(curr_date.month()).style(Style::default().fg(Color::LightCyan));
                h += 2;
            }

            // Add day and update index j
            if curr_date.weekday() == Weekday::Sun {
                curr_month.height += 1;
                h += 1;
            }
            curr_month.days.push(DayWidget::new(curr_date).style(Style::default().fg(Color::White)));

            curr_date = curr_date.succ();
        }

        if curr_month.days.len() > 0 {
            months.push(curr_month);
        }

        let today = Local::now().date();

        CalendarWidget {
            months,
            selected,
            today,
            style: Style::default(),
            block: None,
            highlight_style: Style::default(),
            highlight_symbol: None,
            first_date,
            last_date: curr_date.pred(),
        }
    }

    pub fn get_date(&self) -> Date<Local> {
        self.selected
    }

    pub fn get_last_date(&self) -> Date<Local> {
        self.last_date
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

        if block_area.width < 1 || block_area.height < 1 {
            return;
        }

        if self.months.is_empty() {
            return;
        }

        let mut prev_month_height: usize = 0;
        let mut last_day_x = block_area.x;
        for (i, month) in self
            .months
            .iter()
            .enumerate()
        {
            let (x, y) = (block_area.left(), block_area.top() + prev_month_height as u16);
            let middle = (x + block_area.width) / 2;
            let area = Rect {
                x,
                y,
                width: block_area.width,
                height: month.height() as u16,
            };
            buf.set_style(area, month.style);
            //buf.set_style(area, Style::default().bg(Color::Rgb(x as u8 * 30, 4 * y as u8, 10 * i as u8)));


            buf.set_spans(middle - month.name.len() as u16 / 2, area.y,
                &Spans::from(vec![
                    Span::raw(month.name)
                ]),
                area.width
            );

            let mut last_day_y = 1;
            for (j, day) in month
                .days
                .iter()
                .enumerate()
            {
                let area = Rect {
                    x: last_day_x + 1,
                    y: y + last_day_y,
                    width: 2,
                    height: 1,
                };
                buf.set_style(area, day.style);
                buf.set_spans(area.x, area.y, day.content.lines.get(0).unwrap(), area.width);

                if day.date == self.selected {
                    buf.set_style(area, self.highlight_style);
                }
                if day.date == self.today {
                    buf.set_style(area, Style::default().fg(Color::Red));
                }

                let next_pos = area.x + area.width;
                if next_pos / block_area.width > 0 {
                    last_day_x = 1;
                    last_day_y += 1;
                } else {
                    last_day_x = next_pos;
                }

            }
            prev_month_height += month.height();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::Calendar;

}
