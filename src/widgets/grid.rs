use tui::{widgets::{canvas::{Canvas, Line, Context, Shape, Painter}, Block}, layout::Rect, style::Color};

pub struct Grid {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub color: Color,
}

impl Shape for Grid {
    fn draw(&self, painter: &mut Painter) {
        let (x1, y1) = match painter.get_point(self.x1, self.y1) {
            Some(c) => c,
            None => return,
        };
        let (x2, y2) = match painter.get_point(self.x2, self.y2) {
            Some(c) => c,
            None => return,
        };
        for line in (0..=24) {
            let h = y1 / 23;
            for x in x1..(x2 - 5) {
                painter.paint(x, line * h, self.color);
            }
        }
    }
}

