use std::collections::HashSet;

use super::{
    segment::Segment,
    starship::{
        Starship,
        starship_getter::{
            starship_get_rotation, starship_get_x, starship_get_x_speed, starship_get_y,
            starship_get_y_speed,
        },
    },
};

pub const MARS_GRAVITY: f64 = 3.711;
const MAX_H_SPEED_ON_LAND: f32 = 20.;
const MAX_V_SPEED_ON_LAND: f32 = 40.;
const ANGLE_TO_LAND: i32 = 0;

pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Game {
    nb_points: usize,
    points: Vec<Point>,
    segments: Vec<Segment>,
    crash_points: [u32; 7000], // Dans le puzzle lv 3 cela ne marchera plus
    landing: Segment,
}

impl Game {
    pub fn new(nb_points: usize) -> Self {
        let points = Vec::with_capacity(nb_points);
        let segments = Vec::with_capacity(nb_points);
        Game {
            nb_points,
            points,
            segments,
            crash_points: [0; 7000],
            landing: Segment {
                start: Point { x: 0, y: 0 },
                end: Point { x: 0, y: 0 },
            },
        }
    }

    pub fn starship_is_crash(&self, starship: Starship) -> bool {
        let x = starship_get_x(starship);
        let y = starship_get_y(starship);
        self.crash_points[x as usize] >= y
    }

    pub fn starship_is_landing(&self, starship: Starship) -> bool {
        let x = starship_get_x(starship) as usize;
        let y = starship_get_y(starship) as usize;
        x >= self.landing.start.x
            && x <= self.landing.end.x
            && y >= self.landing.start.y
            && y <= self.landing.end.y
            && starship_get_x_speed(starship).abs() <= MAX_H_SPEED_ON_LAND
            && starship_get_y_speed(starship).abs() <= MAX_V_SPEED_ON_LAND
            && starship_get_rotation(starship) == ANGLE_TO_LAND
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
            let end = Point { x, y };
            self.add_segment(Segment { start, end });
        }
    }

    fn add_segment(&mut self, seg: Segment) {
        if seg.is_landing() {
            self.landing = Segment {
                start: Point {
                    x: seg.start.x,
                    y: seg.start.y,
                },
                end: Point {
                    x: seg.end.x,
                    y: seg.end.y,
                },
            };
        } else {
            let start_x = seg.start.x as u32;
            let start_y = seg.start.y.min(seg.end.y) as u32;
            let end_y = seg.start.y.max(seg.end.y) as u32;
            let end_x = seg.end.x as u32;
            let ecart_x = end_x - start_x as u32;
            let ecart_y = end_y - seg.start.y as u32;
            self.segments.push(seg);
            for x in start_x..=end_x {
                self.crash_points[x as usize] = ((x * ecart_x).max(1) / ecart_y.max(1)) + start_y;
            }
        }
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::starship::{Starship, starship_init};

    #[test]
    fn test_game() {
        let mut game = Game::new(10);
        game.add_point(0, 0);
        game.add_point(1, 1);
        game.add_point(2, 2);
        game.add_point(3, 3);

        assert_eq!(game.points.len(), 4);
        assert_eq!(game.segments.len(), 2);
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

        let starship = starship_init(1000, 2000, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_crash(starship));
        let starship = starship_init(1000, 1500, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_crash(starship));
        let starship = starship_init(2001, 500, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_crash(starship));
        let starship = starship_init(3499, 500, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_crash(starship));
        let starship = starship_init(5000, 1500, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_crash(starship));
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

        let starship = starship_init(1000, 2000, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_landing(starship));
        let starship = starship_init(1000, 1500, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_landing(starship));
        let starship = starship_init(2000, 500, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_landing(starship));
        let starship = starship_init(3500, 500, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_landing(starship));
        let starship = starship_init(5000, 1500, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_landing(starship));
    }
}
