use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Spans,
    text::Text,
    widgets::{
        self,
        canvas::{Canvas, Line},
        Block, Borders, StatefulWidget, Widget,
    },
};

use crate::{app::InputMode, event::Event};

use super::{event_slot::EventSlot, time_grid::TimeGrid};

#[derive(Debug)]
pub(crate) struct EventViewState {
    pub selected: Option<usize>,
    pub events: Vec<Event>,
}

impl EventViewState {
    pub fn new(selected: Option<usize>, events: Vec<Event>) -> Self {
        EventViewState { selected, events }
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }
}

pub(crate) struct EventView<'a> {
    events: Vec<Event>,
    style: Style,
    block: Option<Block<'a>>,
    highlight_style: Style,
    highlight_symbol: Option<&'a str>,
    enhanced: bool,
}

impl<'a> EventView<'a> {
    pub fn new(events: Vec<Event>, input_mode: &InputMode, enhanced_graphics: bool) -> Self {
        let events_len = events.len();
        EventView {
            events,
            block: None,
            style: Style::default(),
            highlight_symbol: None,
            highlight_style: Style::default(),
            enhanced: enhanced_graphics,
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
    type State = EventViewState;

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

        let tg = TimeGrid::new(self.enhanced).style(Style::default().fg(Color::Red));
        tg.render(block_area, buf);

        for (i, event) in self.events.iter().enumerate() {
            let style = if state.selected.is_some() && state.selected.unwrap() == i {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Blue)
            };

            let slot = EventSlot::new(event, style);
            slot.render(block_area, buf);
        }
    }
}
