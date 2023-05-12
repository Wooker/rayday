use std::ops::Mul;

use chrono::Timelike;
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
    event: &'a Event,
    style: Style,
}

impl<'a> EventSlot<'a> {
    pub fn new(event: &'a Event, style: Style) -> Self {
        EventSlot { event, style }
    }
}

impl<'a> Widget for EventSlot<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let start_datetime = self.event.time().start_datetime();
        let end_datetime = self.event.time().end_datetime();

        let start: f64 = start_datetime.hour() as f64 + start_datetime.minute() as f64 / 60f64;
        let end: f64 = end_datetime.hour() as f64 + end_datetime.minute() as f64 / 60f64;

        let duration = end - start;

        let r = Rect {
            x: area.left(),
            y: area.top() + (start as f64 * area.height as f64 / 24.0).floor() as u16,
            width: area.width - 5,
            height: (duration as f64 * area.height as f64 / 24.0).ceil() as u16,
        };

        let text = format!(
            "{} ({}-{})",
            self.event.desc(),
            start_datetime.format("%R").to_string(),
            end_datetime.format("%R").to_string()
        );

        let canvas = Canvas::default()
            .x_bounds([0.0, (area.width + 5).into()])
            .y_bounds([-48.0, 0.0]) // twice as the bounds of the time grid to fit in the middle
            .paint(|ctx| {
                let rect = &Rectangle {
                    x: 0.0,
                    y: -start * 2.0,
                    width: area.width as f64,
                    height: -duration * 2.0,
                    color: self.style.fg.unwrap(),
                };
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
