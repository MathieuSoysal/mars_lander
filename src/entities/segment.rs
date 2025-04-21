use super::game::Point;

pub struct Segment {
    pub start: Point,
    pub end: Point,
}

impl Segment {
    pub fn new(start: Point, end: Point) -> Self {
        Segment { start, end }
    }

    pub fn length(&self) -> f64 {
        let dx = self.end.x as f64 - self.start.x as f64;
        let dy = self.end.y as f64 - self.start.y as f64;
        dx.hypot(dy)
    }

    pub fn is_landing(&self) -> bool {
        self.start.y == self.end.y
    }
}
