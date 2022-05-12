use tui::{
    layout::Rect,
    text::{Spans, Span},
    buffer::Buffer,
    widgets::{StatefulWidget, Widget, Block, canvas::{Line, Canvas, Rectangle}}, text::Text, style::{Style, Color}
};

pub struct EventSlot<'a> {
    start: f64,
    duration: f64,
    color: Color,
    content: Text<'a>,
    desc: String,
}

impl<'a> EventSlot<'a> {
    pub fn new<T>(desc: String, start: f64, duration: f64, content: T, color: Color) -> Self
    where
        T: Into<Text<'a>>,
    {
        EventSlot {
            desc,
            start,
            duration,
            color,
            content: content.into(),
        }
    }
}

impl<'a> Widget for EventSlot<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let canvas = Canvas::default()
            .x_bounds([0.0, area.width.into()])
            .y_bounds([-240.0, 0.0])
            .paint(|ctx| {
                let desc = self.desc.clone();
                ctx.draw(&Rectangle {
                    x: 1.0,
                    y: -self.start,
                    width: (area.width - 7) as f64,
                    height: self.duration,
                    color: self.color,
                });
                ctx.layer();
                ctx.print(area.width as f64 / 2.0, -self.start + self.duration / 2.0, Spans::from(vec![
                    Span::raw(desc),
                ]));
            });
        canvas.render(area, buf);
        //buf.set_style(area, Style::default().bg(Color::Gray));
        //buf.set_string(area.x + 1, area.y + 1, self.desc, Style::default().fg(Color::Red));
    }
}
