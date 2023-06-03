use std::ops::Mul;

use centered_interval_tree::InnerInfo;
use chrono::{NaiveTime, Timelike};
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

pub struct EventSlot {
    info: InnerInfo<NaiveTime, String>,
    style: Style,
}

impl EventSlot {
    pub fn new(info: InnerInfo<NaiveTime, String>, style: Style) -> Self {
        EventSlot { info, style }
    }
}

impl Widget for EventSlot {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let interval = self.info.interval();

        let start: f64 = interval.0.hour() as f64 + interval.0.minute() as f64 / 60f64;
        let end: f64 = interval.1.hour() as f64 + interval.1.minute() as f64 / 60f64;

        let duration = end - start;

        let r = Rect {
            x: area.left(),
            y: area.top() + (start as f64 * area.height as f64 / 24.0).floor() as u16,
            width: area.width - 5,
            height: (duration as f64 * area.height as f64 / 24.0).ceil() as u16,
        };

        let text = format!(
            "{} ({}-{})",
            self.info.value(),
            interval.0.format("%R").to_string(),
            interval.1.format("%R").to_string(),
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
