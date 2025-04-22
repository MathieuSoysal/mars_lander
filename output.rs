use entities::{
    game::Game,
    genome::{gen_init_rand, get_power_on_turn, get_rotate_on_turn, DNA},
    starship::{self, Starship},
};
use genetics::sim::{seq::Simulator, Builder, Simulation};
pub mod entities {
    pub mod genome {
        use super::{
            game::{self, Game},
            starship::{self, Starship},
        };
        use crate::genetics::pheno::Phenotype;
        const GEN_POWER_SIZE_BITS: usize = 2;
        const GEN_ROTATE_SIZE_BITS: usize = 2;
        const GENOME_SIZE: usize = 100;
        type Nucleotide = u8;
        pub type Genome = [Nucleotide; GENOME_SIZE];
        pub fn gen_init_rand() -> Genome {
            let mut genome = [0; GENOME_SIZE];
            for i in 0..GENOME_SIZE {
                genome[i] = rand::random::<u8>();
            }
            genome
        }
        pub fn get_rotate_on_turn(genome: &Genome, nb_turn: usize) -> i8 {
            let turn = genome[nb_turn];
            let rotate = turn & GEN_ROTATE_SIZE_BITS as u8;
            match rotate {
                0 => 0,
                1 => -15,
                2 => 15,
                _ => 0,
            }
        }
        pub fn get_power_on_turn(genome: &Genome, nb_turn: usize) -> i8 {
            let turn = genome[nb_turn];
            let power = (turn >> GEN_ROTATE_SIZE_BITS) & GEN_POWER_SIZE_BITS as u8;
            match power {
                0 => 0,
                1 => -1,
                2 => 1,
                _ => 0,
            }
        }
        #[derive(Clone, Copy)]
        pub struct DNA<'a> {
            genome: Genome,
            game: &'a Game,
            starship: Starship,
        }
        impl<'a> DNA<'a> {
            pub fn new(genome: Genome, game: &'a Game, starship: Starship) -> Self {
                DNA {
                    genome,
                    game,
                    starship,
                }
            }
            pub fn get_genome(&self) -> &Genome {
                &self.genome
            }
            pub fn get_starship(&self) -> &Starship {
                &self.starship
            }
        }
        impl<'a> Phenotype<i32> for DNA<'a> {
            fn fitness(&self) -> i32 {
                let mut s = self.starship.copy();
                let disntance = self.game.get_distance_to_landing(&s);
                for i in 0..GENOME_SIZE {
                    let rotate = get_rotate_on_turn(&self.genome, i);
                    let power = get_power_on_turn(&self.genome, i);
                    s.add_power(power as i32);
                    s.add_rotation(rotate);
                    s.apply_movement();
                    if self.game.starship_is_crash(&s) {
                        return disntance - self.game.get_distance_to_landing(&s);
                    }
                    if self.game.starship_is_landing(&s) {
                        return s.get_fuel() as i32 + 2000;
                    }
                }
                self.game.get_distance_to_landing(&s)
            }
            fn mutate(&self) -> DNA<'a> {
                let mut mutated = *self;
                for i in 0..GENOME_SIZE {
                    if rand::random::<bool>() {
                        mutated.genome[i] = rand::random::<u8>();
                    }
                }
                mutated
            }
            fn crossover(&self, other: &Self) -> Self {
                let mut child = *self;
                let crossover_point = rand::random::<usize>() % GENOME_SIZE;
                for i in crossover_point..GENOME_SIZE {
                    child.genome[i] = other.genome[i];
                }
                child
            }
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::entities::game::Game;
            use crate::entities::starship::Starship;
            use crate::genetics::sim::select::UnstableMaximizeSelector;
            use crate::genetics::sim::seq::Simulator;
            use crate::genetics::sim::{Builder, Simulation};
            #[test]
            fn test_genome() {
                let mut game = Game::new(10);
                game.add_point(0, 1500);
                game.add_point(1000, 2000);
                game.add_point(2000, 500);
                game.add_point(3500, 500);
                game.add_point(5000, 1500);
                game.add_point(6999, 1000);
                let starship = Starship::new(2500, 2700, 550, 0, 0, 0., 0.);
                let mut population: Vec<DNA> = Vec::with_capacity(300);
                let mut rng = ::rand::thread_rng();
                for _ in 0..300 {
                    let genome = gen_init_rand();
                    let dna = DNA::new(genome, &game, starship.copy());
                    population.push(dna);
                }
                #[allow(deprecated)]
                let mut builder = Simulator::builder(&mut population);
                builder
                    .with_selector(Box::new(UnstableMaximizeSelector::new(100)))
                    .with_max_iters(200);
                let mut s = builder.build();
                s.run();
                let result = s.get().unwrap();
                let time = s.time();
                println!("Execution time: {} ns.", time.unwrap());
                println!(
                    "Result: {:?} | Fitness: {}.",
                    result.get_genome(),
                    result.fitness()
                );
            }
        }
    }
    mod segment {
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
    }
    pub mod game {
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
            crash_points: [u32; 7000],
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
                let d_start_x = self.landing.start.x as i32 - x;
                let d_end_x = self.landing.end.x as i32 - x;
                if d_end_x < d_start_x {
                    d_end_x as i32
                } else {
                    d_start_x as i32
                }
            }
            pub fn starship_is_crash(&self, starship: &Starship) -> bool {
                let x = starship.get_x();
                let y = starship.get_y();
                x < 0
                    || x > MAX_X
                    || y > MAX_Y
                    || y < 0
                    || self.crash_points[x as usize] >= y as u32
            }
            pub fn starship_is_landing(&self, starship: &Starship) -> bool {
                let x = starship.get_x() as usize;
                let y = starship.get_y() as usize;
                x >= self.landing.start.x
                    && x <= self.landing.end.x
                    && y >= self.landing.start.y
                    && y <= self.landing.end.y
                    && starship.get_x_speed().abs() <= MAX_H_SPEED_ON_LAND
                    && starship.get_y_speed().abs() <= MAX_V_SPEED_ON_LAND
                    && starship.get_rotation() == ANGLE_TO_LAND as i8
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
                        self.crash_points[x as usize] =
                            ((x * ecart_x).max(1) / ecart_y.max(1)) + start_y;
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
            use crate::entities::starship::Starship;
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
                let starship = Starship::new(1000, 2000, 0, 0, 0, 0., 0.);
                assert!(game.starship_is_crash(&starship));
                let starship = Starship::new(1000, 1500, 0, 0, 0, 0., 0.);
                assert!(game.starship_is_crash(&starship));
                let starship = Starship::new(2001, 500, 0, 0, 0, 0., 0.);
                assert!(!game.starship_is_crash(&starship));
                let starship = Starship::new(3499, 500, 0, 0, 0, 0., 0.);
                assert!(!game.starship_is_crash(&starship));
                let starship = Starship::new(5000, 1500, 0, 0, 0, 0., 0.);
                assert!(game.starship_is_crash(&starship));
                let starship = Starship::new(9999, 1000, 0, 0, 0, 0., 0.);
                assert!(game.starship_is_crash(&starship));
                let starship = Starship::new(5, 9999, 0, 0, 0, 0., 0.);
                assert!(game.starship_is_crash(&starship));
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
    }
    pub mod simulation {
        pub mod simulator {}
    }
    pub mod starship {
        mod physics {
            use super::Starship;
            const MARS_GRAVITY: f32 = 3.711;
            impl Starship {
                #[inline(always)]
                pub fn apply_movement(&mut self) {
                    let rad = (self.rotation as f32).to_radians();
                    let thrust = if (self.power as u16) <= self.fuel {
                        self.fuel -= self.power as u16;
                        self.power as f32
                    } else {
                        self.power = 0;
                        0.0
                    };
                    let v0_x = self.x_speed;
                    let v0_y = self.y_speed;
                    let v1_x = -rad.sin() * thrust;
                    let v1_y = rad.cos() * thrust - MARS_GRAVITY;
                    self.add_x_speed(v1_x);
                    self.add_y_speed(v1_y);
                    self.add_x(v0_x + v1_x * 0.5);
                    self.add_y(v0_y + v1_y * 0.5);
                }
            }
            #[cfg(test)]
            mod tests {
                use super::*;
                #[test]
                fn test_apply_movement() {
                    let mut starship = Starship::new(1000, 2700, 10000, 0, 0, 0., 0.);
                    for _ in 0..20 {
                        starship.apply_movement();
                    }
                    assert_eq!(starship.get_y(), 1958);
                }
                #[test]
                fn test_apply_movement_with_power1_with_orientation() {
                    let mut starship = Starship::new(1000, 2700, 10000, 0, 0, 0., 0.);
                    starship.add_power(1);
                    starship.add_rotation(15);
                    for _ in 0..20 {
                        starship.apply_movement();
                    }
                    assert_eq!(starship.get_y(), 2151);
                }
                #[test]
                fn test_apply_movement_with_power4_with_orientation() {
                    let mut starship = Starship::new(2500, 2700, 10000, 0, 0, 0., 0.);
                    starship.add_rotation(15);
                    for _ in 0..20 {
                        starship.add_power(1);
                        starship.apply_movement();
                    }
                    assert_eq!(starship.get_y(), 2621);
                    assert_eq!(starship.get_x(), 2322);
                }
                #[test]
                fn test_apply_movement_with_power4_with_orientation_negatif() {
                    let mut starship = Starship::new(2500, 2700, 10000, 0, 0, 0., 0.);
                    starship.add_rotation(-15);
                    for _ in 0..20 {
                        starship.add_power(1);
                        starship.apply_movement();
                    }
                    assert_eq!(starship.get_y(), 2621);
                    assert_eq!(starship.get_x(), 2678);
                }
            }
        }
        pub mod starship_adder {
            use super::*;
            const MIN_SPEED: f32 = -500.;
            const MAX_SPEED: f32 = 500.;
            impl Starship {
                #[inline(always)]
                pub fn add_rotation(&mut self, rotation: i8) {
                    let new_rotation = self.rotation + rotation;
                    if new_rotation < MIN_ROTATE {
                        self.rotation = MIN_ROTATE;
                    } else if new_rotation > MAX_ROTATE {
                        self.rotation = MAX_ROTATE;
                    } else {
                        self.rotation = new_rotation;
                    }
                }
                #[inline(always)]
                pub fn add_x(&mut self, x: f32) {
                    self.x += (x * 100.) as i32;
                }
                #[inline(always)]
                pub fn add_y(&mut self, y: f32) {
                    self.y += (y * 100.) as i32;
                }
                #[inline(always)]
                pub fn add_x_speed(&mut self, x_speed: f32) {
                    let new_x_speed = self.x_speed + x_speed;
                    if new_x_speed < MIN_SPEED {
                        self.x_speed = MIN_SPEED;
                    } else if new_x_speed > MAX_SPEED {
                        self.x_speed = MAX_SPEED;
                    } else {
                        self.x_speed = new_x_speed;
                    }
                }
                #[inline(always)]
                pub fn add_y_speed(&mut self, y_speed: f32) {
                    let new_y_speed = self.y_speed + y_speed;
                    if new_y_speed < MIN_SPEED {
                        self.y_speed = MIN_SPEED;
                    } else if new_y_speed > MAX_SPEED {
                        self.y_speed = MAX_SPEED;
                    } else {
                        self.y_speed = new_y_speed;
                    }
                }
                #[inline(always)]
                pub fn add_power(&mut self, add_power: i32) {
                    if add_power > 0 && self.power < MAX_POWER {
                        self.power += 1 as u8;
                    } else if add_power < 0 && self.power > 0 {
                        self.power -= 1 as u8;
                    }
                }
            }
            #[cfg(test)]
            mod tests {
                use super::*;
                #[test]
                fn test_starship_add_rotation() {
                    let mut starship = Starship::new(0, 0, 0, 0, 0, 0., 0.);
                    starship.add_rotation(10);
                    assert_eq!(starship.get_rotation(), 10);
                    starship.add_rotation(-20);
                    assert_eq!(starship.get_rotation(), -10);
                    starship.add_rotation(MAX_ROTATE + 30);
                    assert_eq!(starship.get_rotation(), MAX_ROTATE);
                }
                #[test]
                fn test_starship_add_x_speed() {
                    let mut starship = Starship::new(0, 0, 0, 0, 0, 0., 0.);
                    starship.add_x_speed(50.);
                    assert_eq!(starship.get_x_speed(), 50.);
                    starship.add_x_speed(-100.);
                    assert_eq!(starship.get_x_speed(), -50.);
                    starship.add_x_speed(MAX_SPEED * 2.);
                    assert_eq!(starship.get_x_speed(), MAX_SPEED);
                }
                #[test]
                fn test_starship_add_y_speed() {
                    let mut starship = Starship::new(0, 0, 0, 0, 0, 0., 0.);
                    starship.add_y_speed(30.);
                    assert_eq!(starship.get_y_speed(), 30.);
                    starship.add_y_speed(-50.);
                    assert_eq!(starship.get_y_speed(), -20.);
                    starship.add_y_speed(MAX_SPEED * 2.);
                    assert_eq!(starship.get_y_speed(), MAX_SPEED);
                }
                #[test]
                fn test_starship_add_power() {
                    let mut starship = Starship::new(0, 0, 50, 0, 0, 0., 0.);
                    starship.add_power(10);
                    assert_eq!(starship.get_power(), 1);
                    starship.add_power(-5);
                    assert_eq!(starship.get_power(), 0);
                    starship.add_power(MAX_POWER as i32 + 1);
                    assert_eq!(starship.get_power(), 1);
                    starship.add_power(0 as i32 - 1);
                    assert_eq!(starship.get_power(), 0);
                }
            }
        }
        #[derive(Clone, Copy)]
        pub struct Starship {
            pub x: i32,
            pub y: i32,
            pub fuel: u16,
            pub rotation: i8,
            pub power: u8,
            pub x_speed: f32,
            pub y_speed: f32,
        }
        const MIN_ROTATE: i8 = -90;
        const MAX_ROTATE: i8 = 90;
        const MAX_POWER: u8 = 4;
        impl Starship {
            pub fn copy(&self) -> Self {
                Starship {
                    x: self.x,
                    y: self.y,
                    fuel: self.fuel,
                    rotation: self.rotation,
                    power: self.power,
                    x_speed: self.x_speed,
                    y_speed: self.y_speed,
                }
            }
            pub fn new(
                x: i32,
                y: i32,
                fuel: u16,
                rotation: i8,
                power: u8,
                x_speed: f32,
                y_speed: f32,
            ) -> Self {
                Starship {
                    x: (x * 100),
                    y: (y * 100),
                    fuel,
                    rotation,
                    power,
                    x_speed,
                    y_speed,
                }
            }
            #[inline(always)]
            pub fn get_x(&self) -> i32 {
                (self.x + 50) / 100
            }
            #[inline(always)]
            pub fn get_y(&self) -> i32 {
                (self.y + 50) / 100
            }
            #[inline(always)]
            pub fn get_fuel(&self) -> u16 {
                self.fuel
            }
            #[inline(always)]
            pub fn get_rotation(&self) -> i8 {
                self.rotation
            }
            #[inline(always)]
            pub fn get_power(&self) -> u8 {
                self.power
            }
            #[inline(always)]
            pub fn get_x_speed(&self) -> f32 {
                self.x_speed
            }
            #[inline(always)]
            pub fn get_y_speed(&self) -> f32 {
                self.y_speed
            }
        }
    }
}
pub mod genetics {
    extern crate rand;
    pub mod pheno {
        pub trait Fitness: Ord + Eq {
            fn zero() -> Self;
            fn abs_diff(&self, other: &Self) -> Self;
        }
        pub trait Phenotype<F>: Clone
        where
            F: Fitness,
        {
            fn fitness(&self) -> F;
            fn crossover(&self, other: &Self) -> Self;
            fn mutate(&self) -> Self;
        }
    }
    pub mod sim {
        use super::pheno;
        use pheno::{Fitness, Phenotype};
        mod earlystopper {
            use super::iterlimit::*;
            use crate::genetics::pheno;
            use pheno::Fitness;
            #[derive(Copy, Clone, Debug)]
            pub struct EarlyStopper<F: Fitness> {
                delta: F,
                previous: F,
                iter_limit: IterLimit,
            }
            impl<F: Fitness> EarlyStopper<F> {
                pub fn new(delta: F, n_iters: u64) -> EarlyStopper<F> {
                    EarlyStopper {
                        delta,
                        previous: F::zero(),
                        iter_limit: IterLimit::new(n_iters),
                    }
                }
                pub fn update(&mut self, fitness: F) {
                    if self.previous.abs_diff(&fitness) < self.delta {
                        self.previous = fitness;
                        self.iter_limit.inc();
                    } else {
                        self.iter_limit.reset();
                    }
                }
                pub fn reached(&self) -> bool {
                    self.iter_limit.reached()
                }
            }
        }
        mod iterlimit {
            #[derive(Copy, Clone, Debug)]
            pub struct IterLimit {
                max: u64,
                cur: u64,
            }
            impl IterLimit {
                pub fn new(max: u64) -> IterLimit {
                    IterLimit { max, cur: 0 }
                }
                pub fn inc(&mut self) {
                    self.cur += 1;
                }
                pub fn reached(&self) -> bool {
                    self.cur >= self.max
                }
                pub fn reset(&mut self) {
                    self.cur = 0;
                }
                pub fn get(&self) -> u64 {
                    self.cur
                }
            }
        }
        pub mod select {
            mod max {
                use super::*;
                use crate::genetics::pheno;
                use pheno::{Fitness, Phenotype};
                #[derive(Clone, Copy, Debug)]
                #[deprecated(
                    note = "The `MaximizeSelector` has bad performance due to sorting. For better performance with potentially different results, \
                   use the `UnstableMaximizeSelector`.",
                    since = "1.7.7"
                )]
                pub struct MaximizeSelector {
                    count: usize,
                }
                impl MaximizeSelector {
                    pub fn new(count: usize) -> MaximizeSelector {
                        MaximizeSelector { count }
                    }
                }
                impl<T, F> Selector<T, F> for MaximizeSelector
                where
                    T: Phenotype<F>,
                    F: Fitness,
                {
                    fn select<'a>(&self, population: &'a [T]) -> Result<Parents<&'a T>, String> {
                        if self.count == 0
                            || self.count % 2 != 0
                            || self.count * 2 >= population.len()
                        {
                            return Err(format!(
                                "Invalid parameter `count`: {}. Should be larger than zero, a \
                 multiple of two and less than half the population size.",
                                self.count
                            ));
                        }
                        let mut borrowed: Vec<&T> = population.iter().collect();
                        borrowed.sort_by(|x, y| y.fitness().cmp(&x.fitness()));
                        let mut index = 0;
                        let mut result: Parents<&T> = Vec::new();
                        while index < self.count {
                            result.push((borrowed[index], borrowed[index + 1]));
                            index += 2;
                        }
                        Ok(result)
                    }
                }
            }
            mod max_unstable {
                use super::*;
                use crate::genetics::pheno;
                use pheno::{Fitness, Phenotype};
                #[derive(Clone, Copy, Debug)]
                pub struct UnstableMaximizeSelector {
                    count: usize,
                }
                impl UnstableMaximizeSelector {
                    pub fn new(count: usize) -> UnstableMaximizeSelector {
                        UnstableMaximizeSelector { count }
                    }
                }
                impl<T, F> Selector<T, F> for UnstableMaximizeSelector
                where
                    T: Phenotype<F>,
                    F: Fitness,
                    T: Send,
                    T: Sync,
                {
                    fn select<'a>(&self, population: &'a [T]) -> Result<Parents<&'a T>, String> {
                        if self.count == 0
                            || self.count % 2 != 0
                            || self.count * 2 >= population.len()
                        {
                            return Err(format!(
                                "Invalid parameter `count`: {}. Should be larger than zero, a \
                 multiple of two and less than half the population size.",
                                self.count
                            ));
                        }
                        let mut borrowed: Vec<&T> = population.iter().collect();
                        borrowed.sort_unstable_by(|x, y| y.fitness().cmp(&x.fitness()));
                        let mut index = 0;
                        let mut result: Parents<&T> = Vec::new();
                        while index < self.count {
                            result.push((borrowed[index], borrowed[index + 1]));
                            index += 2;
                        }
                        Ok(result)
                    }
                }
            }
            mod stochastic {
                use super::*;
                use pheno::{Fitness, Phenotype};
                use rand::Rng;
                #[derive(Clone, Copy, Debug)]
                pub struct StochasticSelector {
                    count: usize,
                }
                impl StochasticSelector {
                    pub fn new(count: usize) -> StochasticSelector {
                        StochasticSelector { count }
                    }
                }
                impl<T, F> Selector<T, F> for StochasticSelector
                where
                    T: Phenotype<F>,
                    F: Fitness,
                {
                    fn select<'a>(&self, population: &'a [T]) -> Result<Parents<&'a T>, String> {
                        if self.count == 0 || self.count % 2 != 0 || self.count >= population.len()
                        {
                            return Err(format!(
                                "Invalid parameter `count`: {}. Should be larger than zero, a \
                 multiple of two and less than the population size.",
                                self.count
                            ));
                        }
                        let ratio = population.len() / self.count;
                        let mut result: Parents<&T> = Vec::new();
                        let mut i = ::rand::thread_rng().gen_range(0..population.len());
                        let mut selected = 0;
                        while selected < self.count {
                            result.push((
                                &population[i],
                                &population[(i + ratio - 1) % population.len()],
                            ));
                            i += ratio - 1;
                            i %= population.len();
                            selected += 2;
                        }
                        Ok(result)
                    }
                }
            }
            mod tournament {
                use super::*;
                use pheno::{Fitness, Phenotype};
                use rand::Rng;
                #[derive(Copy, Clone, Debug)]
                pub struct TournamentSelector {
                    count: usize,
                    participants: usize,
                }
                impl TournamentSelector {
                    #[deprecated(
                        note = "The `TournamentSelector` requires at least 2 participants. This is not enforced
                       by the `new` function. You should use `new_checked` instead.",
                        since = "1.7.11"
                    )]
                    pub fn new(count: usize, participants: usize) -> TournamentSelector {
                        TournamentSelector {
                            count,
                            participants,
                        }
                    }
                    pub fn new_checked(
                        count: usize,
                        participants: usize,
                    ) -> Result<TournamentSelector, String> {
                        if count == 0 || count % 2 != 0 || participants < 2 {
                            Err (String :: from ("count must be larger than zero and a multiple of two; participants must be larger than one" ,))
                        } else {
                            Ok(TournamentSelector {
                                count,
                                participants,
                            })
                        }
                    }
                }
                impl<T, F> Selector<T, F> for TournamentSelector
                where
                    T: Phenotype<F>,
                    F: Fitness,
                {
                    fn select<'a>(&self, population: &'a [T]) -> Result<Parents<&'a T>, String> {
                        if self.count == 0
                            || self.count % 2 != 0
                            || self.count * 2 >= population.len()
                        {
                            return Err(format!(
                                "Invalid parameter `count`: {}. Should be larger than zero, a \
                 multiple of two and less than half the population size.",
                                self.count
                            ));
                        }
                        if self.participants == 0 || self.participants >= population.len() {
                            return Err(format!(
                                "Invalid parameter `participants`: {}. Should be larger than \
                 zero and less than the population size.",
                                self.participants
                            ));
                        }
                        let mut result: Parents<&T> = Vec::new();
                        let mut rng = ::rand::thread_rng();
                        for _ in 0..(self.count / 2) {
                            let mut tournament: Vec<&T> = Vec::with_capacity(self.participants);
                            for _ in 0..self.participants {
                                let index = rng.gen_range(0..population.len());
                                tournament.push(&population[index]);
                            }
                            tournament.sort_by(|x, y| y.fitness().cmp(&x.fitness()));
                            result.push((tournament[0], tournament[1]));
                        }
                        Ok(result)
                    }
                }
            }
            #[allow(deprecated)]
            pub use self::max::MaximizeSelector;
            pub use self::max_unstable::UnstableMaximizeSelector;
            pub use self::stochastic::StochasticSelector;
            pub use self::tournament::TournamentSelector;
            use crate::genetics::pheno;
            use pheno::{Fitness, Phenotype};
            use std::fmt::Debug;
            pub type Parents<T> = Vec<(T, T)>;
            pub trait Selector<T, F>: Debug
            where
                T: Phenotype<F>,
                F: Fitness,
            {
                fn select<'a>(&self, population: &'a [T]) -> Result<Parents<&'a T>, String>;
            }
        }
        pub mod seq {
            use super::earlystopper::*;
            use super::iterlimit::*;
            use super::select::*;
            use super::*;
            use pheno::Fitness;
            use pheno::Phenotype;
            use rand::Rng;
            use std::marker::PhantomData;
            use std::time::Instant;
            #[doc = " A sequential implementation of `::sim::Simulation`."]
            #[doc = " The genetic algorithm is run in a single thread."]
            #[derive(Debug)]
            pub struct Simulator<'a, T, F>
            where
                T: 'a + Phenotype<F>,
                F: Fitness,
            {
                population: &'a mut Vec<T>,
                iter_limit: IterLimit,
                selector: Box<dyn Selector<T, F>>,
                earlystopper: Option<EarlyStopper<F>>,
                duration: Option<NanoSecond>,
                error: Option<String>,
                phantom: PhantomData<&'a T>,
            }
            impl<'a, T, F> Simulation<'a, T, F> for Simulator<'a, T, F>
            where
                T: Phenotype<F>,
                F: Fitness,
            {
                type B = SimulatorBuilder<'a, T, F>;
                #[doc = " Create builder."]
                #[allow(deprecated)]
                fn builder(population: &'a mut Vec<T>) -> SimulatorBuilder<'a, T, F> {
                    SimulatorBuilder {
                        sim: Simulator {
                            population,
                            iter_limit: IterLimit::new(100),
                            selector: Box::new(MaximizeSelector::new(3)),
                            earlystopper: None,
                            duration: Some(0),
                            error: None,
                            phantom: PhantomData::default(),
                        },
                    }
                }
                fn step(&mut self) -> StepResult {
                    let time_start;
                    if self.population.is_empty() {
                        self.error = Some(
                            "Tried to run a simulator without a population, or the \
                 population was empty."
                                .to_string(),
                        );
                        return StepResult::Failure;
                    }
                    let should_stop = match self.earlystopper {
                        Some(ref x) => self.iter_limit.reached() || x.reached(),
                        None => self.iter_limit.reached(),
                    };
                    if !should_stop {
                        time_start = Instant::now();
                        let mut children: Vec<T>;
                        {
                            let parents = match self.selector.select(self.population) {
                                Ok(parents) => parents,
                                Err(e) => {
                                    self.error = Some(e);
                                    return StepResult::Failure;
                                }
                            };
                            children = parents
                                .iter()
                                .map(|&(a, b)| a.crossover(b).mutate())
                                .collect();
                        }
                        self.kill_off(children.len());
                        self.population.append(&mut children);
                        if let Some(ref mut stopper) = self.earlystopper {
                            let highest_fitness = self
                                .population
                                .iter()
                                .max_by_key(|x| x.fitness())
                                .unwrap()
                                .fitness();
                            stopper.update(highest_fitness);
                        }
                        self.iter_limit.inc();
                        self.duration = match self.duration {
                            Some(x) => {
                                let elapsed = time_start.elapsed();
                                let y = elapsed.as_secs() as NanoSecond * 1_000_000_000
                                    + u64::from(elapsed.subsec_nanos()) as NanoSecond;
                                Some(x + y)
                            }
                            None => None,
                        };
                        StepResult::Success
                    } else {
                        StepResult::Done
                    }
                }
                #[allow(deprecated)]
                fn checked_step(&mut self) -> StepResult {
                    if self.error.is_some() {
                        panic!("Attempt to step a Simulator after an error!")
                    } else {
                        self.step()
                    }
                }
                #[allow(deprecated)]
                fn run(&mut self) -> RunResult {
                    loop {
                        match self.step() {
                            StepResult::Success => {}
                            StepResult::Failure => return RunResult::Failure,
                            StepResult::Done => return RunResult::Done,
                        }
                    }
                }
                fn get(&'a self) -> SimResult<'a, T> {
                    match self.error {
                        Some(ref e) => Err(e),
                        None => Ok(self.population.iter().max_by_key(|x| x.fitness()).unwrap()),
                    }
                }
                fn iterations(&self) -> u64 {
                    self.iter_limit.get()
                }
                fn time(&self) -> Option<NanoSecond> {
                    self.duration
                }
                fn population(&self) -> Vec<T> {
                    self.population.clone()
                }
            }
            impl<'a, T, F> Simulator<'a, T, F>
            where
                T: Phenotype<F>,
                F: Fitness,
            {
                #[doc = " Kill off phenotypes using stochastic universal sampling."]
                fn kill_off(&mut self, count: usize) {
                    let ratio = self.population.len() / count;
                    let mut i = ::rand::thread_rng().gen_range(0..self.population.len());
                    for _ in 0..count {
                        self.population.swap_remove(i);
                        i += ratio;
                        i %= self.population.len();
                    }
                }
            }
            #[doc = " A `Builder` for the `Simulator` type."]
            #[derive(Debug)]
            pub struct SimulatorBuilder<'a, T, F>
            where
                T: 'a + Phenotype<F>,
                F: Fitness,
            {
                sim: Simulator<'a, T, F>,
            }
            impl<'a, T, F> SimulatorBuilder<'a, T, F>
            where
                T: Phenotype<F>,
                F: Fitness,
            {
                #[doc = " Set the selector of the resulting `Simulator`."]
                #[doc = ""]
                #[doc = " Returns itself for chaining purposes."]
                #[deprecated(
                    note = "The consuming builder functions may be removed in a future release.
                       Use the functions that start with `with_` instead.",
                    since = "1.8.0"
                )]
                pub fn set_selector(mut self, sel: Box<dyn Selector<T, F>>) -> Self {
                    self.sim.selector = sel;
                    self
                }
                #[doc = " Set the selector of the resulting `Simulator`."]
                #[doc = ""]
                #[doc = " Returns a mutable reference to itself for chaining purposes."]
                #[doc = " Does not consume the builder."]
                pub fn with_selector(&mut self, sel: Box<dyn Selector<T, F>>) -> &mut Self {
                    self.sim.selector = sel;
                    self
                }
                #[doc = " Set the maximum number of iterations of the resulting `Simulator`."]
                #[doc = ""]
                #[doc = " The `Simulator` will stop running after this number of iterations."]
                #[doc = ""]
                #[doc = " Returns itself for chaining purposes."]
                #[deprecated(
                    note = "The consuming builder functions may be removed in a future release.
                       Use the functions that start with `with_` instead.",
                    since = "1.8.0"
                )]
                pub fn set_max_iters(mut self, i: u64) -> Self {
                    self.sim.iter_limit = IterLimit::new(i);
                    self
                }
                #[doc = " Set the maximum number of iterations of the resulting `Simulator`."]
                #[doc = ""]
                #[doc = " The `Simulator` will stop running after this number of iterations."]
                #[doc = ""]
                #[doc = " Returns a mutable reference to itself for chaining purposes."]
                #[doc = " Does not consume the builder."]
                pub fn with_max_iters(&mut self, i: u64) -> &mut Self {
                    self.sim.iter_limit = IterLimit::new(i);
                    self
                }
                #[doc = " Set early stopping. If for `n_iters` iterations, the change in the highest fitness"]
                #[doc = " is smaller than `delta`, the simulator will stop running."]
                #[doc = ""]
                #[doc = " Returns itself for chaining purposes."]
                #[deprecated(
                    note = "The consuming builder functions may be removed in a future release.
                       Use the functions that start with `with_` instead.",
                    since = "1.8.0"
                )]
                pub fn set_early_stop(mut self, delta: F, n_iters: u64) -> Self {
                    self.sim.earlystopper = Some(EarlyStopper::new(delta, n_iters));
                    self
                }
                #[doc = " Set early stopping. If for `n_iters` iterations, the change in the highest fitness"]
                #[doc = " is smaller than `delta`, the simulator will stop running."]
                #[doc = ""]
                #[doc = " Returns a mutable reference to itself for chaining purposes."]
                #[doc = " Does not consume the builder."]
                pub fn with_early_stop(&mut self, delta: F, n_iters: u64) -> &mut Self {
                    self.sim.earlystopper = Some(EarlyStopper::new(delta, n_iters));
                    self
                }
            }
            impl<'a, T, F> Builder<Simulator<'a, T, F>> for SimulatorBuilder<'a, T, F>
            where
                T: Phenotype<F>,
                F: Fitness,
            {
                fn build(self) -> Simulator<'a, T, F> {
                    self.sim
                }
            }
        }
        pub mod types {
            use crate::genetics::pheno;
            use pheno::Fitness;
            macro_rules ! implement_fitness_int { ($ ($ t : ty) ,*) => { $ (impl Fitness for $ t { fn zero () -> $ t { 0 } fn abs_diff (& self , other : &$ t) -> $ t { if self > other { self - other } else { other - self } } }) * } }
            implement_fitness_int!(i8, i16, i32, i64, u8, u16, u32, u64, usize);
        }
        pub trait Builder<T: ?Sized> {
            fn build(self) -> T
            where
                T: Sized;
        }
        pub type NanoSecond = i64;
        pub type SimResult<'a, T> = Result<&'a T, &'a str>;
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum StepResult {
            Success,
            Failure,
            Done,
        }
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum RunResult {
            Failure,
            Done,
        }
        pub trait Simulation<'a, T, F>
        where
            T: Phenotype<F>,
            F: Fitness,
        {
            type B: Builder<Self>;
            fn builder(population: &'a mut Vec<T>) -> Self::B
            where
                Self: Sized;
            fn run(&mut self) -> RunResult;
            #[deprecated(
                note = "To encourage checking the `StepResult` while maintaining backwards \
                compatibility, this function has been deprecated in favour of `checked_step`.",
                since = "1.7.0"
            )]
            fn step(&mut self) -> StepResult;
            fn checked_step(&mut self) -> StepResult;
            fn get(&'a self) -> SimResult<'a, T>;
            fn time(&self) -> Option<NanoSecond>;
            fn iterations(&self) -> u64;
            fn population(&self) -> Vec<T>;
        }
    }
}
pub fn get_next_move(game: Game, starship: Starship) -> (i8, i8) {
    let mut population: Vec<DNA> = Vec::with_capacity(300);
    let mut rng = ::rand::thread_rng();
    for _ in 0..300 {
        let genome = gen_init_rand();
        let dna = DNA::new(genome, &game, starship);
        population.push(dna);
    }
    #[allow(deprecated)]
    let mut builder = Simulator::builder(&mut population);
    builder
        .with_selector(Box::new(
            genetics::sim::select::UnstableMaximizeSelector::new(10),
        ))
        .with_max_iters(50);
    let mut s = builder.build();
    s.run();
    let result = s.get().unwrap();
    let rotation = get_rotate_on_turn(result.get_genome(), 0);
    let thrust = get_power_on_turn(result.get_genome(), 0);
    (rotation, thrust)
}
fn main() {
    let mut game = entities::game::Game::new(10);
    game.add_point(0, 1500);
    game.add_point(1000, 2000);
    game.add_point(2000, 500);
    game.add_point(3500, 500);
    game.add_point(5000, 1500);
    game.add_point(6999, 1000);
    let starship = entities::starship::Starship::new(2500, 2700, 550, 0, 0, 0., 0.);
    let (rotation, thrust) = get_next_move(game, starship);
    println!("{} {}", rotation, thrust);
}

