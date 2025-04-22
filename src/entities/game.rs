use super::{segment::Segment, starship::Starship};

pub const MARS_GRAVITY: f64 = 3.711;
const MAX_H_SPEED_ON_LAND: f32 = 20.;
const MAX_V_SPEED_ON_LAND: f32 = 40.;
const ANGLE_TO_LAND: i32 = 0;
const MAX_X: i32 = 6999;
const MAX_Y: i32 = 2999;

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

    pub fn get_distance_to_landing(&self, starship: &Starship) -> i32 {
        let x = starship.get_x();
        if x >= self.landing.start.x as i32
            && x <= self.landing.end.x as i32
        {
            return 0;
        }
        let d_start_x = (self.landing.start.x as i32 - x).abs();
        let d_end_x = (self.landing.end.x as i32 - x).abs();
        if d_end_x < d_start_x {
            d_end_x as i32
        } else {
            d_start_x as i32
        }
    }

    pub fn starship_is_crash(&self, starship: &Starship) -> bool {
        let x = starship.get_x();
        let y = starship.get_y();
        let is_on_landing = x as usize >= self.landing.start.x
            && x as usize <= self.landing.end.x
            && y as usize <= self.landing.start.y;
        starship.get_x() < 0
            || x > MAX_X
            || y > MAX_Y
            || starship.get_y() < 0
            || self.crash_points[x as usize] >= y as u32
            || is_on_landing
    }

    pub fn starship_is_landing(&self, starship: &Starship) -> bool {
        let x = starship.get_x() as usize;
        let y = starship.get_y() as usize;
        x >= self.landing.start.x
            && x <= self.landing.end.x
            && y <= self.landing.start.y
            && starship.get_x_speed().abs() <= MAX_H_SPEED_ON_LAND
            && starship.get_y_speed().abs() <= MAX_V_SPEED_ON_LAND + 30.
            && starship.get_rotation() == ANGLE_TO_LAND as i8
    }

    pub fn add_point(&mut self, x: usize, y: usize) {
        self.points.push(Point { x, y });
        if self.points.len() > 1 {
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
            let start_x = seg.start.x as i32;
            let start_y = seg.start.y as i32;
            let end_y = seg.end.y as i32;
            let end_x = seg.end.x as i32;
            self.segments.push(seg);
            for x in start_x..=end_x {
                self.crash_points[x as usize] = ((((end_y - start_y) * x) / (end_x - start_x))
                    + (start_y + -1 * ((start_x * (end_y - start_y)) / (end_x - start_x))))
                    as u32;
            }
        }
    }

    pub fn to_svg(&self) -> String {
        let mut svg = String::new();
        for segment in &self.segments {
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"red\"  stroke-width=\"7\" />\n",
                segment.start.x, 3000 - segment.start.y, segment.end.x, 3000 - segment.end.y
            ));
        }
        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"green\"  stroke-width=\"8\">\n",
            self.landing.start.x,
            3000 - self.landing.start.y,
            self.landing.end.x,
            3000 - self.landing.end.y
        ));
        svg.push_str("<title>Landing</title>\n");
        svg.push_str("</line>\n");
        svg
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
        assert!(game.starship_is_crash(&starship));
        let starship = Starship::new(1000, 1500, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_crash(&starship));
        let starship = Starship::new(5000, 1500, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_crash(&starship));
        let starship = Starship::new(9999, 1000, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_crash(&starship));
        let starship = Starship::new(5, 9999, 0, 0, 0, 0., 0.);
        assert!(game.starship_is_crash(&starship));
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
        assert!(!game.starship_is_crash(&starship));
        let starship = Starship::new(600, 1850, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_crash(&starship));
        let starship = Starship::new(1600, 1200, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_crash(&starship));
        let starship = Starship::new(6000, 1500, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_crash(&starship));
        let starship = Starship::new(1500, 1500, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_crash(&starship));
        let starship = Starship::new(500, 2000, 0, 0, 0, 0., 0.);
        assert!(!game.starship_is_crash(&starship));
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
