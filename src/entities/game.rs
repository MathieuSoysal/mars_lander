use super::starship::Starship;

pub const MARS_GRAVITY: f64 = 3.711;
const MAX_H_SPEED_ON_LAND: f32 = 20.;
const MAX_V_SPEED_ON_LAND: f32 = 40.;
const ANGLE_TO_LAND: i32 = 0;
const MAX_X: i32 = 6999;
const MAX_Y: i32 = 2999;
pub const WIDTH: usize = 7000;
pub const HEIGHT: usize = 3000;

#[derive(Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

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

pub struct Game {
    points: Vec<Point>,
    segments: Vec<Segment>,
    landing: Segment,
}

fn orientation(p: &Point, q: &Point, r: &Point) -> i32 {
    let val = (q.y as i32 - p.y as i32) * (r.x as i32 - q.x as i32)
        - (q.x as i32 - p.x as i32) * (r.y as i32 - q.y as i32);
    if val == 0 {
        0 // colinear
    } else if val > 0 {
        1 // clockwise
    } else {
        2 // counter-clockwise
    }
}

fn on_segment(p: &Point, q: &Point, r: &Point) -> bool {
    q.x >= p.x.min(r.x) && q.x <= p.x.max(r.x) && q.y >= p.y.min(r.y) && q.y <= p.y.max(r.y)
}

fn collide(seg1: &Segment, seg2: &Segment) -> bool {
    let p1 = &seg1.start;
    let q1 = &seg1.end;
    let p2 = &seg2.start;
    let q2 = &seg2.end;

    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special cases
    if o1 == 0 && on_segment(p1, p2, q1) {
        return true;
    }
    if o2 == 0 && on_segment(p1, q2, q1) {
        return true;
    }
    if o3 == 0 && on_segment(p2, p1, q2) {
        return true;
    }
    if o4 == 0 && on_segment(p2, q1, q2) {
        return true;
    }

    false
}

impl Game {
    pub fn new(nb_points: usize) -> Self {
        let points = Vec::with_capacity(nb_points);
        let segments = Vec::with_capacity(nb_points);
        Game {
            points,
            segments,
            landing: Segment {
                start: Point { x: 0, y: 0 },
                end: Point { x: 0, y: 0 },
            },
        }
    }

    pub fn get_distance_to_landing(&self, starship: &Starship) -> (i32, i32) {
        let x = starship.get_x();
        let y = starship.get_y();

        let dist_x = if x >= self.landing.start.x as i32 && x <= self.landing.end.x as i32 {
            0
        } else {
            let d_start_x = (self.landing.start.x as i32 - x).abs();
            let d_end_x = (self.landing.end.x as i32 - x).abs();
            d_end_x.min(d_start_x) as i32
        };

        let dist_y = if y >= self.landing.start.y as i32 && y <= self.landing.end.y as i32 {
            0
        } else {
            let d_start_y = (self.landing.start.y as i32 - y).abs();
            let d_end_y = (self.landing.end.y as i32 - y).abs();
            d_end_y.min(d_start_y) as i32
        };
        (dist_x, dist_y)
    }

    fn collide_seg(&self, starship: &Starship, px: i32, py: i32) -> bool {
        let cx = starship.get_x();
        let cy = starship.get_y();

        let segment = Segment {
            start: Point {
                x: px as usize,
                y: py as usize,
            },
            end: Point {
                x: cx as usize,
                y: cy as usize,
            },
        };

        self.segments.iter().any(|seg| collide(seg, &segment))
    }

    pub fn starship_is_crash(&self, starship: &Starship, px: i32, py: i32) -> bool {
        let x = starship.get_x();
        let y = starship.get_y();
        let is_on_landing = x as usize >= self.landing.start.x
            && x as usize <= self.landing.end.x
            && y as usize <= self.landing.start.y;
        x < 0
            || x > MAX_X
            || y > MAX_Y
            || y < 0
            || self.collide_seg(starship, px, py)
            || is_on_landing
    }

    pub fn starship_is_landing(&self, starship: &Starship) -> bool {
        let x = starship.get_x() as usize;
        let y = starship.get_y() as usize;
        x >= self.landing.start.x
            && x <= self.landing.end.x
            && y <= self.landing.start.y
            && starship.get_x_speed().abs() <= MAX_H_SPEED_ON_LAND
            && starship.get_y_speed().abs() <= MAX_V_SPEED_ON_LAND
            && starship.get_rotation() == ANGLE_TO_LAND as i8
    }

    pub fn add_point(&mut self, x: usize, y: usize) {
        if !self.points.is_empty() {
            let start = self.points.last().unwrap().clone();
            let end = Point { x, y };
            self.add_segment(Segment { start, end });
        }
        self.points.push(Point { x, y });
    }

    fn add_segment(&mut self, seg: Segment) {
        if seg.is_landing() {
            self.landing = seg.clone();
        } else {
            self.segments.push(seg);
        }
    }

    pub fn to_svg(&self) -> String {
        let mut svg = String::new();
        for segment in &self.segments {
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"red\" stroke-width=\"7\" />\n",
                segment.start.x, HEIGHT - segment.start.y, segment.end.x, HEIGHT - segment.end.y
            ));
        }
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"green\" stroke-width=\"8\">\n",
            self.landing.start.x,
            HEIGHT - self.landing.start.y,
            self.landing.end.x,
            HEIGHT - self.landing.end.y
        ));
        svg.push_str("<title>Landing</title>\n");
        svg.push_str("</line>\n");
        svg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::starship::Starship;

    #[test]
    fn test_game() {
        let mut game = Game::new(10);
        game.add_point(0, 0);
        game.add_point(1, 1);
        game.add_point(2, 2);
        game.add_point(3, 3);

        assert_eq!(game.points.len(), 4);
        assert_eq!(game.segments.len(), 3);
    }

    #[test]
    fn test_starship_is_crash() {
        let mut game = Game::new(10);
        game.add_point(0, 1500);
        game.add_point(1000, 2000);
        game.add_point(2000, 500);
        game.add_point(3500, 500);
        game.add_point(5000, 1500);
        game.add_point(6999, 1000);

        let starship = Starship::new(1000, 2000, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_crash(&starship, 0, 0));
        let starship = Starship::new(1000, 1500, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_crash(&starship, 0, 0));
        let starship = Starship::new(5000, 1500, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_crash(&starship, 0, 0));
        let starship = Starship::new(9999, 1000, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_crash(&starship, 0, 0));
        let starship = Starship::new(5, 9999, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_crash(&starship, 0, 0));
    }

    #[test]
    fn test_starship_is_not_crash() {
        let mut game = Game::new(10);
        game.add_point(0, 1500);
        game.add_point(1000, 2000);
        game.add_point(2000, 500);
        game.add_point(3500, 500);
        game.add_point(5000, 1500);
        game.add_point(6999, 1000);

        let starship = Starship::new(500, 2500, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_crash(&starship, starship.get_x(), starship.get_y()));
        let starship = Starship::new(600, 1850, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_crash(&starship, starship.get_x(), starship.get_y()));
        let starship = Starship::new(1600, 1200, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_crash(&starship, starship.get_x(), starship.get_y()));
        let starship = Starship::new(6000, 1500, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_crash(&starship, starship.get_x(), starship.get_y()));
        let starship = Starship::new(1500, 1500, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_crash(&starship, starship.get_x(), starship.get_y()));
        let starship = Starship::new(500, 2000, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_crash(&starship, starship.get_x(), starship.get_y()));
    }

    #[test]
    fn test_starship_is_landing() {
        let mut game = Game::new(10);
        game.add_point(0, 1500);
        game.add_point(1000, 2000);
        game.add_point(2000, 500);
        game.add_point(3500, 500);
        game.add_point(5000, 1500);
        game.add_point(6999, 1000);

        let starship = Starship::new(1000, 2000, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_landing(&starship));
        let starship = Starship::new(1000, 1500, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_landing(&starship));
        let starship = Starship::new(2000, 500, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_landing(&starship));
        let starship = Starship::new(3500, 500, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_landing(&starship));
        let starship = Starship::new(5000, 1500, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_landing(&starship));
    }
}
