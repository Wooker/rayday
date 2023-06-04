use chrono::{DateTime, Local, NaiveTime};
use std::ops::Bound::*;
use store_interval_tree::{Interval, IntervalTree};
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
    event_tree: IntervalTree<NaiveTime, String>,
    style: Style,
    block: Option<Block<'a>>,
    highlight_style: Style,
    highlight_symbol: Option<&'a str>,
    enhanced: bool,
}

impl<'a> EventView<'a> {
    pub fn new(events: Vec<Event>, input_mode: &InputMode, enhanced_graphics: bool) -> Self {
        let mut tree = IntervalTree::<NaiveTime, String>::new();

        for event in events.iter() {
            let time = event.time();
            let interval = Interval::new(
                Included(time.start_datetime()),
                Excluded(time.end_datetime()),
            );
            tree.insert(interval, event.desc());
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

        let whole_day: Interval<NaiveTime> = Interval::new(
            Included(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            Included(NaiveTime::from_hms_opt(23, 59, 59).unwrap()),
        );

        let mut event_slots = Vec::<(EventSlot, usize)>::new();
        let mut max_layer = 0;

        for (i, entry) in self.event_tree.query(&whole_day).enumerate() {
            let entry_interval = entry.interval();
            let mut interval: (NaiveTime, NaiveTime) = (
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            );

            if let Included(start_time) = entry_interval.low() {
                interval.0 = start_time.to_owned();
            }

            if let Excluded(end_time) = entry_interval.high() {
                interval.1 = end_time.to_owned();
            }

            let layer = self
                .event_tree
                .intervals_between(&Interval::point(interval.0), &Interval::point(interval.1))
                .len();

            max_layer = max_layer.max(layer);
            // dbg!(&max_layer);

            let style = if state.selected.is_some() && state.selected.unwrap() == i {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Blue)
            };

            let slot = EventSlot::new(entry, style);
            event_slots.push((slot, layer));
        }

        let chunks = Layout::default().direction(Direction::Horizontal);

        let mut constraints = vec![];
        for _ in 0..max_layer {
            constraints.push(Constraint::Ratio(1, (max_layer) as u32));
        }

        let chunks = chunks.constraints(constraints).split(block_area);

        let tg = TimeGrid::new(self.enhanced).style(Style::default().fg(Color::Red));
        tg.render(block_area, buf);

        for (slot, layer) in event_slots.into_iter() {
            slot.render(chunks[max_layer - layer], buf);
        }
    }
}
