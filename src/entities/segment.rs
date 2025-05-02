use super::game::Point;

#[derive(Clone)]
pub struct Segment {
    pub start: Point,
    pub end: Point,
}

impl Segment {

    pub fn is_landing(&self) -> bool {
        self.start.y == self.end.y
    }
}
