use std::collections::HashSet;

use super::starship::{
    Starship,
    starship_getter::{starship_get_x, starship_get_x_speed, starship_get_y, starship_get_y_speed},
};

pub const MARS_GRAVITY: f64 = 3.711;

pub struct Point {
    x: usize,
    y: usize,
}

pub struct Segment {
    start: Point,
    end: Point,
}

pub struct Game {
    nb_points: usize,
    points: Vec<Point>,
    segments: Vec<Segment>,
    map: HashSet<(usize, usize)>,
}

impl Game {
    pub fn new(nb_points: usize) -> Self {
        let points = Vec::with_capacity(nb_points);
        let segments = Vec::with_capacity(nb_points);
        Game {
            nb_points,
            points,
            segments,
            map: HashSet::new(),
        }
    }

    pub fn add_point(&mut self, x: usize, y: usize) {
        if self.points.len() < self.nb_points {
            self.points.push(Point { x, y });
        }
        if self.points.len() % 2 == 0 {
            let start = Point {
                x: self.points[self.points.len() - 2].x,
                y: self.points[self.points.len() - 2].y,
            };
            let end = Point {
                x: self.points[self.points.len() - 1].x,
                y: self.points[self.points.len() - 1].y,
            };
            self.segments.push(Segment { start, end });
        }
    }

    fn add_segment(&mut self, start: Point, end: Point) {
        self.segments.push(Segment { start, end });
        // each pixels under the segment is now flipped
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                self.flip_bit(x, y);
            }
        }
    }

    fn flip_bit(&mut self, x: usize, y: usize) {
        if self.map.contains(&(x, y)) {
            self.map.remove(&(x, y));
        } else {
            self.map.insert((x, y));
        }
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}
impl Segment {
    pub fn new(start: Point, end: Point) -> Self {
        Segment { start, end }
    }

    pub fn length(&self) -> f64 {
        let dx = (self.end.x as f64 - self.start.x as f64).powi(2);
        let dy = (self.end.y as f64 - self.start.y as f64).powi(2);
        (dx + dy).sqrt()
    }

    pub fn is_landing(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn collision_projectile_segment(
        &self,
        starship: Starship,
        initial_x: f64,
        initial_y: f64,
    ) -> bool {
        let x = starship_get_x(starship) as f64;
        let y = starship_get_y(starship) as f64;

        let dx = self.end.x as f64 - self.start.x as f64;
        let dy = self.end.y as f64 - self.start.y as f64;

        let a = dx.powi(2) + dy.powi(2);
        let b = 2.0 * (dx * (initial_x - x) + dy * (initial_y - y));
        let c = (initial_x - x).powi(2) + (initial_y - y).powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return false;
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

        t1 >= 0.0 && t1 <= 1.0 || t2 >= 0.0 && t2 <= 1.0
    }
}
