use chrono::{Date, Local, Datelike};
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

pub struct MonthWidget<'a> {
    days: Vec<DayWidget<'a>>,
    content: Text<'a>,
    style: Style,
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
        }
    }

    fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

pub struct CalendarWidget<'a> {
    months: Vec<MonthWidget<'a>>,
    date: (usize, usize), // (month, day)
    style: Style,
    block: Option<Block<'a>>,
    highlight_style: Style,
    highlight_symbol: Option<&'a str>,
}

impl<'a> CalendarWidget<'a> {
    pub fn new((days, (month, day)): (Vec<Date<Local>>, (usize, usize))) -> Self {
        let mut months: Vec<MonthWidget> = Vec::new();
        let mut curr_month = MonthWidget::new(days.get(0).unwrap().month().to_string());

        for day in days.iter() {
            if (day.day() == 1 && !curr_month.days.is_empty()) {
                months.push(curr_month);
                curr_month = MonthWidget::new(day.month().to_string());
            }
            curr_month.days.push(DayWidget::new(day.day().to_string()));
        }

        CalendarWidget {
            months,
            date: (month, day),
            style: Style::default(),
            block: None,
            highlight_style: Style::default(),
            highlight_symbol: None,
        }
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

impl<'a> Widget for CalendarWidget<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
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

        //let empty_space = list_area.top().checked_sub(self.items.len() as u16 / 7).unwrap_or(list_area.top());
        //let (start, end) = self.get_items_bounds(state.selected, state.offset, 1);
        for (i, month) in self
            .months
            .iter()
            .enumerate()
        {
            // 3 = (2 + 1), 2 - width of date, 1 - space between dates
            let (x, y) = (block_area.left() + 1, block_area.top());
            let area = Rect {
                x: x + 3 * (i as u16 % 7),
                y: y + i as u16 / 7, // 7 is num of days in a week
                width: block_area.width, // 2 is date width
                height: 1,
            };
            buf.set_style(area, month.style);
            buf.set_spans(area.x, area.y, month.content.lines.get(0).unwrap(), area.width);

            for (j, day) in month
                .days
                .iter()
                .enumerate()
            {
                //let is_selected = state.selected.map(|s| s == i).unwrap_or(false);
                for (j, day) in month.days.iter().enumerate() {
                    buf.set_spans(area.x, area.y, day.content.lines.get(0).unwrap(), area.width);
                }
                if (i, j) == self.date {
                    buf.set_style(area, self.highlight_style);
                }
            }
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct ListState {
    offset: usize,
    selected: Option<usize>,
}

impl ListState {
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListItem<'a> {
    content: Text<'a>,
    style: Style,
}

impl<'a> ListItem<'a> {
    pub fn new<T>(content: T) -> ListItem<'a>
    where
        T: Into<Text<'a>>,
    {
        ListItem {
            content: content.into(),
            style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> ListItem<'a> {
        self.style = style;
        self
    }

    pub fn height(&self) -> usize {
        self.content.height()
    }
}

#[derive(Debug, Clone)]
pub struct List<'a> {
    block: Option<Block<'a>>,
    items: Vec<ListItem<'a>>,

    chosen: usize,

    style: Style,
    start_corner: Corner,

    highlight_style: Style,

    highlight_symbol: Option<&'a str>,

    repeat_highlight_symbol: bool,
}

impl<'a> List<'a> {
    pub fn new<T>(items: T) -> List<'a>
    where
        T: Into<Vec<ListItem<'a>>>,
    {
        List {
            block: None,
            style: Style::default(),
            chosen: 0,
            items: items.into(),
            start_corner: Corner::TopLeft,
            highlight_style: Style::default(),
            highlight_symbol: None,
            repeat_highlight_symbol: false,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> List<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> List<'a> {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> List<'a> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> List<'a> {
        self.highlight_style = style;
        self
    }

    pub fn repeat_highlight_symbol(mut self, repeat: bool) -> List<'a> {
        self.repeat_highlight_symbol = repeat;
        self
    }

    pub fn start_corner(mut self, corner: Corner) -> List<'a> {
        self.start_corner = corner;
        self
    }

    fn get_items_bounds(
        &self,
        selected: Option<usize>,
        offset: usize,
        max_height: usize,
    ) -> (usize, usize) {
        let offset = offset.min(self.items.len().saturating_sub(1));
        let mut start = offset;
        let mut end = offset;

        let mut height = if self.items.len() % 7 == 0 {
            self.items.len() / 7
        } else {
            self.items.len() / 7 + 1
        };

        let selected = selected.unwrap_or(0).min(self.items.len() - 1);
        /*
        while selected >= end {
            height = height.saturating_add(self.items[end].height());
            end += 1;
            while height > max_height {
                height = height.saturating_sub(self.items[start].height());
                start += 1;
            }
        }
        while selected < start {
            start -= 1;
            height = height.saturating_add(self.items[start].height());
            while height > max_height {
                end -= 1;
                height = height.saturating_sub(self.items[end].height());
            }
        }
        */
        (0, self.items.len())
    }
}

impl<'a> StatefulWidget for List<'a> {
    type State = ListState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        if self.items.is_empty() {
            return;
        }

        let empty_space = list_area.top().checked_sub(self.items.len() as u16 / 7).unwrap_or(list_area.top());
        let (start, end) = self.get_items_bounds(state.selected, state.offset, 1);
        for (i, item) in self
            .items
            .iter_mut()
            .enumerate()
            .skip(state.offset)
            .take(end - start)
        {
            // 3 = (2 + 1), 2 - width of date, 1 - space between dates
            let (x, y) = (list_area.left() + 1, list_area.top().checked_add(empty_space / 2).unwrap());
            let area = Rect {
                x: x + 3 * (i as u16 % 7),
                y: y + i as u16 / 7, // 7 is num of days in a week
                width: 2, // 2 is date width
                height: item.height() as u16,
            };
            buf.set_style(area, item.style);

            let is_selected = state.selected.map(|s| s == i).unwrap_or(false);
            for (j, line) in item.content.lines.iter().enumerate() {
                buf.set_spans(area.x, area.y, line, area.width);
            }
            if is_selected {
                buf.set_style(area, self.highlight_style);
            }
        }
    }
}

impl<'a> Widget for List<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ListState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}
