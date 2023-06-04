use std::ops::Mul;

use chrono::{NaiveTime, Timelike};
use std::ops::Bound::*;
use std::ops::RangeBounds;

use store_interval_tree::Entry;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Text,
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Context, Line, Rectangle},
        Block, Clear, Paragraph, StatefulWidget, Widget,
    },
};

use crate::event::{Event, EventTime};

pub struct EventSlot<'a> {
    entry: Entry<'a, NaiveTime, String>,
    style: Style,
}

impl<'a> EventSlot<'a> {
    pub fn new(entry: Entry<'a, NaiveTime, String>, style: Style) -> Self {
        EventSlot { entry, style }
    }
}

impl<'a> Widget for EventSlot<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let entry_interval = self.entry.interval();
        let mut interval: (NaiveTime, NaiveTime) = (
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );

        // let start: f64 = interval.low().hour() as f64 + interval.0.minute() as f64 / 60f64;
        let mut start: f64 = 0.0;
        if let Included(start_time) = entry_interval.low() {
            start = start_time.hour() as f64 + start_time.minute() as f64 / 60f64;
            interval.0 = start_time.to_owned();
        }

        let mut end: f64 = 0.0;
        if let Excluded(end_time) = entry_interval.high() {
            end = end_time.hour() as f64 + end_time.minute() as f64 / 60f64;
            interval.1 = end_time.to_owned();
        }

        let duration = end - start;

        let r = Rect {
            x: area.left(),
            y: area.top() + (start as f64 * area.height as f64 / 24.0).floor() as u16,
            width: area.width - 5,
            height: (duration as f64 * area.height as f64 / 24.0).ceil() as u16,
        };

        let mut text = format!(
            "{} ({}-{})",
            self.entry.value(),
            interval.0.format("%R").to_string(),
            interval.1.format("%R").to_string(),
        );

        let rect = &Rectangle {
            x: 0.0,
            y: -start * 2.0,
            width: area.width as f64,
            height: -duration * 2.0,
            color: self.style.fg.unwrap(),
        };
        text.truncate((rect.width as usize / 2) - 1);
        text = format!("{}...", text);
        let canvas = Canvas::default()
            .x_bounds([0.0, (area.width + 5).into()])
            .y_bounds([-48.0, 0.0]) // twice as the bounds of the time grid to fit in the middle
            .paint(|ctx| {
                ctx.draw(rect);
                ctx.layer();
                ctx.print(
                    rect.x + (rect.width / 2.0) - text.len() as f64 / 2.0,
                    rect.y - duration,
                    Spans::from(Span::styled(text.clone(), self.style)),
                );
            });
        &Clear.render(r, buf);
        canvas.render(area, buf);
    }
}
