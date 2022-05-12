use num_traits::cast::FromPrimitive;
use chrono::{Date, Local, Datelike, Weekday, Month};
use tui::{
    buffer::Buffer,
    layout::{Corner, Rect},
    style::{Style, Color},
    text::{Text, Spans},
    widgets::{Block, StatefulWidget, Widget},
};
use unicode_width::UnicodeWidthStr;

const DAY_WIDTH: u8 = 2;

#[derive(Debug, Clone, Default)]
pub struct DayWidget<'a> {
    content: Text<'a>,
    style: Style,
}

impl<'a> DayWidget<'a> {
    fn new<T>(content: T) -> Self
    where
        T: Into<Text<'a>>,
    {
        DayWidget {
            content: content.into(),
            style: Style::default(),
        }
    }
}

#[derive(Debug)]
pub struct MonthWidget<'a> {
    content: Text<'a>,
    style: Style,
    days: Vec<DayWidget<'a>>,
    height: usize,
}

impl<'a> MonthWidget<'a> {
    fn new<T>(content: T) -> Self
    where
        T: Into<Text<'a>>,
    {
        MonthWidget {
            days: Vec::new(),
            content: content.into(),
            style: Style::default(),
            height: 0,
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

// TODO add convenient state struct
#[derive(Debug)]
pub struct CalendarWidget<'a> {
    months: Vec<MonthWidget<'a>>,
    date: (u32, u32), // (month, day)
    today: (u32, u32),
    style: Style,
    block: Option<Block<'a>>,
    highlight_style: Style,
    highlight_symbol: Option<&'a str>,
}

impl<'a> CalendarWidget<'a> {
    pub fn new(days: Vec<Date<Local>>, (mut month, mut day): (u32, u32)) -> Self {
        let mut months: Vec<MonthWidget> = Vec::new();
        let now = Local::now().date();
        let mut today = (0, 0);

        // i, j - new indicies for (month, day)
        let (mut curr_month, mut i) = (MonthWidget::new(Month::from_u32(days.get(0).unwrap().month()).unwrap().name()), 0);
        let mut j: usize = 0;

        let mut first_weekday = Weekday::Mon;
        for (k, curr_day) in days.iter().enumerate() {
            // When new month starts store the month in `months` vec and clear `month`
            if (curr_day.day() == 1 && !curr_month.days.is_empty() || k == days.len() - 1) {
                let mut height = 2 + (curr_month.days.len() / 7);
                if first_weekday != Weekday::Mon {
                    height += 1;
                }
                curr_month.height = height;
                months.push(curr_month);
                i += 1;
                j = 0;
                curr_month = MonthWidget::new(Month::from_u32(curr_day.month()).unwrap().name());
                first_weekday = curr_day.weekday();
            }
            curr_month.days.push(DayWidget::new(curr_day.day().to_string()));
            j += 1;

            // Change the indicies to match `months, days` vectors indicies
            if (curr_day.month(), curr_day.day()) == (month, day) {
                (month, day) = (i, j as u32 - 1);
            }
            if (curr_day.month(), curr_day.day()) == (now.month(), now.day()) {
                today = (i, j as u32 - 1);
            }
        }


        CalendarWidget {
            months,
            date: (month, day),
            today,
            style: Style::default(),
            block: None,
            highlight_style: Style::default(),
            highlight_symbol: None,
        }
    }

    pub fn get_date(&self) -> (u32, u32) {
        self.date
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
    type State = (u32, u32);

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
        let mut last_day_pos = (block_area.x, block_area.y);
        for (i, month) in self
            .months
            .iter()
            .enumerate()
        {
            let (x, y) = (block_area.left(), block_area.top() + prev_month_height as u16);
            let middle = (x + block_area.width) / 2;
            let area = Rect {
                x: middle,
                y,
                width: block_area.width,
                height: month.height() as u16,
            };
            buf.set_style(area, month.style);
            buf.set_spans(area.x, area.y, month.content.lines.get(0).unwrap(), area.width);

            for (j, day) in month
                .days
                .iter()
                .enumerate()
            {
                let area = Rect {
                    x: last_day_pos.0 + 1,
                    y: last_day_pos.1 + 1,
                    width: 2,
                    height: 1,
                };
                buf.set_style(area, day.style);
                buf.set_spans(area.x, area.y, day.content.lines.get(0).unwrap(), area.width);

                if (i as u32, j as u32) == self.date {
                    buf.set_style(area, self.highlight_style);
                } else if (i as u32, j as u32) == self.today {
                    buf.set_style(area, Style::default().fg(Color::Red));
                }

                let next_pos = area.x + area.width;
                if next_pos / block_area.width > 0 {
                    last_day_pos.0 = 1;
                    last_day_pos.1 += 1;
                } else {
                    last_day_pos.0 = next_pos;
                }

            }
            prev_month_height += month.height();
            last_day_pos.1 += 3;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::Calendar;

}
