use std::borrow::Borrow;

use tui::{
    layout::Rect,
    text::Spans,
    buffer::Buffer,
    widgets::{StatefulWidget, Widget, Block}, text::Text, style::Style
};

use crate::event::Event;

pub struct EventState {
    selected: Option<usize>,
}

impl EventState {
    pub fn new(selected: Option<usize>) -> Self {
        EventState { selected }
    }
}

pub struct EventView<'a> {
    events: Vec<Event>,
    state: EventState,
    style: Style,
    block: Option<Block<'a>>,
    highlight_style: Style,
    highlight_symbol: Option<&'a str>,
}

impl<'a> EventView<'a> {
    pub fn new(events: Vec<Event>) -> Self {
        EventView {
            events,
            state: EventState { selected: None },
            block: None,
            style: Style::default(),
            highlight_symbol: None,
            highlight_style: Style::default(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> EventView<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> EventView<'a> {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> EventView<'a> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> EventView<'a> {
        self.highlight_style = style;
        self
    }
}


impl<'a> StatefulWidget for EventView<'a> {
    type State = EventState;

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

        if self.events.is_empty() {
            return;
        }

        for (i, event) in self
            .events
            .iter()
            .enumerate()
        {
            let (x, y) = (block_area.left(), block_area.top());
            let area = Rect {
                x,
                y: y + i as u16,
                width: block_area.width,
                height: 1,
            };
            buf.set_style(area, self.style);
            buf.set_spans(area.x, area.y,Spans::from(event.to_string()).borrow(), area.width);
        }
    }
}
