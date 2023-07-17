use centered_interval_tree::CenteredIntervalTree;
use chrono::{DateTime, Local, NaiveTime};

use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
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
    event_tree: CenteredIntervalTree<NaiveTime, String>,
    style: Style,
    block: Option<Block<'a>>,
    highlight_style: Style,
    highlight_symbol: Option<&'a str>,
    enhanced: bool,
}

impl<'a> EventView<'a> {
    pub fn new(events: Vec<Event>, input_mode: &InputMode, enhanced_graphics: bool) -> Self {
        let mut tree = CenteredIntervalTree::<NaiveTime, String>::new(); //IntervalTree::<NaiveTime, String>::new();

        for event in events.iter() {
            let time = event.time();
            let interval = (time.start_datetime(), time.end_datetime());
            tree.add(
                centered_interval_tree::interval::Interval::new(interval.0, interval.1),
                event.desc(),
            );
        }

        EventView {
            event_tree: tree,
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

        let chunks = Layout::default().direction(Direction::Horizontal);
        let overlaps = self.event_tree.overlaps();
        let mut constraints = vec![];
        for _ in 0..=overlaps {
            constraints.push(Constraint::Ratio(1, (overlaps + 1) as u32));
        }

        let chunks = chunks.constraints(constraints).split(block_area);

        let tg = TimeGrid::new(self.enhanced).style(Style::default().fg(Color::Red));
        tg.render(block_area, buf);

        for (i, (info, layer)) in self.event_tree.iter().enumerate() {
            let style = if state.selected.is_some() && state.selected.unwrap() == i {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Blue)
            };

            let slot = EventSlot::new(info, style);
            slot.render(chunks[layer], buf);
        }
    }
}
