use std::ops::Mul;

use chrono::Timelike;
use tui::{
    layout::Rect,
    text::{Spans, Span},
    buffer::Buffer,
    widgets::{StatefulWidget, Widget, Block, canvas::{Line, Canvas, Rectangle, Context}, Clear, Paragraph}, text::Text, style::{Style, Color}
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

        let start: f64 = start_datetime.hour() as f64 + start_datetime.minute() as f64 / 100f64;
        let end: f64 = end_datetime.hour() as f64 + end_datetime.minute() as f64 / 100f64;

        let duration = end - start;

        let r = Rect {
            x: area.left(),
            y: area.top() + (start as f64 * area.height as f64 / 24.0).floor() as u16,
            width: area.width - 5,
            height: (duration as f64 * area.height as f64 / 24.0).ceil() as u16
        };

        /*
        let debug = Paragraph::new(format!("{} {} {} {}", r.x, r.y, r.width, r.height)).style(Style::default().bg(Color::Blue));
        debug.render(r, buf);
        */

        let canvas = Canvas::default() //.marker(tui::symbols::Marker::Block)
            .x_bounds([0.0, (area.width + 7).into()])
            .y_bounds([-24.0, 0.0])
            .paint(|ctx| {
                let rect = &Rectangle {
                    x: 1.0,
                    y: -start,
                    width: area.width as f64,
                    height: -duration,
                    color: Color::Blue, //self.border_color,
                };
                ctx.draw(rect);
                ctx.layer();
                //ctx.print((area.width as f64 / 2.0) - self.event.desc().len() as f64 / 2.0, -start - duration / 2.0, Spans::from(vec![
                ctx.print(rect.x + (rect.width / 2.0) - self.event.desc().len() as f64 / 2.0, rect.y + (rect.height / 2.0), Spans::from(vec![
                    Span::raw(self.event.desc() + " "),
                    Span::raw(
                        format!("({}-{})",
                            start_datetime.format("%R").to_string(),
                            end_datetime.format("%R").to_string()
                        )
                    )
                ]));
            });
        &Clear.render(r, buf);
        canvas.render(area, buf);
        //buf.set_style(area, Style::default().bg(Color::Gray));
        //buf.set_string(area.x + 1, area.y + 1, self.desc, Style::default().fg(Color::Red));
    }
}
