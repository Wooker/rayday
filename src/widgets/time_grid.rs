use std::ops::Neg;

use tui::{
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{
        self,
        canvas::{Canvas, Context, Line, Painter, Shape},
        Block, Borders, Widget,
    },
};

use super::grid::Grid;

pub struct TimeGrid {
    style: Style,
    enhanced_graphics: bool,
}

impl TimeGrid {
    pub fn new(enhanced_graphics: bool) -> Self {
        TimeGrid {
            style: Style::default(),
            enhanced_graphics,
        }
    }

    pub fn style(mut self, style: Style) -> TimeGrid {
        self.style = style;
        self
    }
}

impl Widget for TimeGrid {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        /*
        let debug = widgets::Paragraph::new(format!("{} {} {} {}", area.x, area.y, area.width, area.height));
        debug.render(area, buf);
        */

        let m = Canvas::default()
            .x_bounds([0.0, area.width.into()])
            .y_bounds([-24.0, 0.0])
            .paint(|ctx| {
                for line in (0..24).rev() {
                    ctx.draw(&Line {
                        x1: 0.0,
                        y1: (-line).into(),
                        x2: (area.width - 5).into(),
                        y2: (-line).into(),
                        color: Color::Yellow,
                    });
                    ctx.print(
                        (area.width - 4).into(),
                        (-line).into(),
                        Spans::from(vec![Span::raw(format!("{:0>2}:00", line))]),
                    );
                }
            })
            .marker(if self.enhanced_graphics {
                tui::symbols::Marker::Braille
            } else {
                tui::symbols::Marker::Dot
            });
        m.render(area, buf);
    }
}
