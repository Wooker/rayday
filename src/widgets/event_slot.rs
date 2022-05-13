use chrono::Timelike;
use tui::{
    layout::Rect,
    text::{Spans, Span},
    buffer::Buffer,
    widgets::{StatefulWidget, Widget, Block, canvas::{Line, Canvas, Rectangle, Context}, Clear}, text::Text, style::{Style, Color}
};

use crate::event::{EventTime, Event};

pub struct EventSlot<'a> {
    event: &'a Event,
    border_color: Color,
    style: Style,
}

impl<'a> EventSlot<'a> {
    pub fn new(event: &'a Event, border_color: Color) -> Self {
        EventSlot {
            event,
            border_color,
            style: Style::default(),
        }
    }
}

impl<'a> Widget for EventSlot<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let start_datetime = self.event.time().start_datetime();
        let end_datetime = self.event.time().end_datetime();

        let start: f64 = (start_datetime.hour() * 10) as f64 + start_datetime.minute() as f64 / 6f64;
        let end: f64 = (end_datetime.hour() * 10) as f64 + end_datetime.minute() as f64 / 6f64;

        let canvas = Canvas::default()
            .x_bounds([0.0, area.width.into()])
            .y_bounds([-240.0, 0.0])
            .paint(|ctx| {
                let duration = end - start;
                ctx.draw(&Rectangle {
                    x: 1.0,
                    y: -start,
                    width: (area.width - 7) as f64,
                    height: -duration,
                    color: self.border_color,
                });
                ctx.layer();
                ctx.print(area.width as f64 / 2.0, -start - duration / 2.0, Spans::from(vec![
                    Span::raw(self.event.desc()),
                ]));
            });
        //&Clear.render(area, buf);
        canvas.render(area, buf);
        //buf.set_style(area, Style::default().bg(Color::Gray));
        //buf.set_string(area.x + 1, area.y + 1, self.desc, Style::default().fg(Color::Red));
    }
}
