use std::{io, ops::Div};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct SimulationParams {
    pub pop_size: usize,
    pub nb_generations: i32,
    pub crossover_rate: f64,
    pub mutation_rate: f64,
    pub elite_rate: f64,
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
    pub fn add_power(&mut self, add_power: i16) {
        if add_power > 0 && self.power < MAX_POWER {
            self.power += 1;
        } else if add_power < 0 && self.power > 0 {
            self.power -= 1;
        }
    }
}

pub const MARS_GRAVITY: f32 = 3.711;
const MAX_H_SPEED_ON_LAND: f32 = 20.;
const MAX_V_SPEED_ON_LAND: f32 = 40.;
const ANGLE_TO_LAND: i32 = 0;
const MAX_X: i32 = 6999;
const MAX_Y: i32 = 2999;

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

#[inline(always)]
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

#[inline(always)]
fn on_segment(p: &Point, q: &Point, r: &Point) -> bool {
    q.x >= p.x.min(r.x) && q.x <= p.x.max(r.x) && q.y >= p.y.min(r.y) && q.y <= p.y.max(r.y)
}

#[inline(always)]
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
            d_end_x.min(d_start_x)
        };

        let dist_y = if y >= self.landing.start.y as i32 && y <= self.landing.end.y as i32 {
            0
        } else {
            let d_start_y = (self.landing.start.y as i32 - y).abs();
            let d_end_y = (self.landing.end.y as i32 - y).abs();
            d_end_y.min(d_start_y)
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

        self.segments.iter().any(|seg| collide(seg, &segment)) // TODO reduce number of segments 
    }

    pub fn starship_is_crash(&self, starship: &Starship, px: i32, py: i32) -> bool {
        let x = starship.get_x();
        let y = starship.get_y();
        let is_on_landing = x as usize >= self.landing.start.x
            && x as usize <= self.landing.end.x
            && y as usize <= self.landing.start.y;
        !(0..=MAX_X).contains(&x)
            || !(0..=MAX_Y).contains(&y)
            || self.collide_seg(starship, px, py)
            || is_on_landing
    }

    pub fn starship_is_landing(&self, starship: &Starship) -> bool {
        let x = starship.get_x() as usize;
        let y = starship.get_y() as usize;
        x >= self.landing.start.x + 100
            && x <= self.landing.end.x - 100
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
}

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

use rand::{distr::Bernoulli, prelude::*, random, rng};

const GEN_POWER_SIZE_BITS: usize = 2;
const GEN_ROTATE_SIZE_BITS: usize = 2;
const GEN_MASK_POWER: u8 = (1 << GEN_POWER_SIZE_BITS) - 1;
const GEN_MASK_ROTATE: u8 = (1 << GEN_ROTATE_SIZE_BITS) - 1;

pub const GENOME_SIZE: usize = 100;

type Nucleotide = u8;
pub type Genome = [Nucleotide; GENOME_SIZE];

pub fn gen_init_rand() -> Genome {
    let mut genome = [0; GENOME_SIZE];
    genome.iter_mut().take(GENOME_SIZE).for_each(|genome| {
        *genome = ((random::<u8>() % 3) << GEN_ROTATE_SIZE_BITS) | (random::<u8>() % 3);
    });
    genome
}

pub fn get_rotate_on_turn(genome: &Genome, nb_turn: usize) -> i8 {
    let turn = genome[nb_turn];
    let rotate = turn & GEN_MASK_ROTATE;
    match rotate {
        0 => 0,
        1 => -15,
        2 => 15,
        _ => 0,
    }
}

pub fn get_power_on_turn(genome: &Genome, nb_turn: usize) -> i8 {
    let turn = genome[nb_turn];
    let power = (turn >> GEN_ROTATE_SIZE_BITS) & GEN_MASK_POWER;
    match power {
        0 => 0,
        1 => -1,
        2 => 1,
        _ => 0,
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
pub struct DNA {
    genome: Genome,
    starship: Starship,
    fitness: f64,
}

impl DNA {
    pub fn new(genome: Genome, starship: Starship) -> Self {
        DNA {
            genome,
            starship,
            fitness: -1.,
        }
    }
}

/*
TODO Adjust weights
Order of importance:
 - land_distance (if we take account of y, put 8000 instead of 7000)
    goal: 0
    range: [0, 7000]
    nval: 7001 (2**13)
 - rotation
    goal: 0
    range: [0, 90]
    nval: 91 (2**7)
- x speed
    goal: abs <= 20
    mx_val: [0, 4 * GENOME_SIZE]
    nval: 4 * GENOME_SIZE + 1 (2**9)
 - y speed
    goal: abs <= 40
    mx_val: [0, 4 * GENOME_SIZE]
    nval: 4 * GENOME_SIZE + 1 (2**9)
 - fuel (take account of it only for thoose who succeed)
    range: [0, 2000]
    nval: 2001 (2**11)
*/
const LAND_DISTANCE_X_WEIGHT: f64 = 20.0;
const LAND_DISTANCE_Y_WEIGHT: f64 = 1.0;
const ROTATION_WEIGHT: f64 = 0.0;
const X_SPEED_WEIGHT: f64 = 4.0;
const Y_SPEED_WEIGHT: f64 = 5.0;
const FUEL_WEIGHT: f64 = 100.0;

pub const WINNING_FITNESS: f64 =
    LAND_DISTANCE_X_WEIGHT + ROTATION_WEIGHT + X_SPEED_WEIGHT + Y_SPEED_WEIGHT;

#[inline(always)]
fn calc_fit(
    land_dist_x: i32,
    land_dist_y: i32,
    rot: i8,
    x_speed: f32,
    y_speed: f32,
    fuel: u16,
) -> f64 {
    (7000.0 - land_dist_x as f64).div(7000.) * LAND_DISTANCE_X_WEIGHT
        + (3000.0 - land_dist_y as f64).div(3000.) * LAND_DISTANCE_Y_WEIGHT
        + (90.0 - rot.abs() as f64).div(90.) * ROTATION_WEIGHT
        + (500.0 - x_speed.abs() as f64).div(500.) * X_SPEED_WEIGHT
        + (500.0 - y_speed.abs() as f64).div(500.) * Y_SPEED_WEIGHT
        + (fuel as f64).div(2000.) * FUEL_WEIGHT
}

impl DNA {
    pub fn fitness(&mut self, game: &Game) -> f64 {
        if self.fitness != -1.0 {
            return self.fitness;
        }
        self.fitness = 0.0;
        let mut s = self.starship.copy();
        for i in 0..GENOME_SIZE {
            let px = s.get_x();
            let py = s.get_y();
            let rotate = get_rotate_on_turn(&self.genome, i);
            let power = get_power_on_turn(&self.genome, i);
            s.add_power(power as i16);
            s.add_rotation(rotate);
            s.apply_movement();

            if game.starship_is_landing(&s) {
                self.fitness = calc_fit(0, 0, 0, 0., 0., s.get_fuel());
                break;
            }

            if game.starship_is_crash(&s, px, py) {
                let (land_dist_x, land_dist_y) = game.get_distance_to_landing(&s);
                self.fitness = calc_fit(
                    land_dist_x,
                    land_dist_y,
                    s.get_rotation(),
                    s.get_x_speed(),
                    s.get_y_speed(),
                    0,
                );
                break;
            }
        }
        self.fitness
    }

    pub fn mutate(&self, mutation_rate: &Bernoulli) -> DNA {
        let mut mutated = *self;
        mutated.fitness = -1.;
        for i in 0..GENOME_SIZE {
            if mutation_rate.sample(&mut rng()) {
                mutated.genome[i] = random::<u8>();
            }
        }
        mutated
    }

    pub fn crossover(&self, other: &Self) -> Self {
        let mut child = *self;
        child.fitness = -1.;
        let crossover_point = random::<u32>() as usize % GENOME_SIZE;
        for i in crossover_point..GENOME_SIZE {
            child.genome[i] = other.genome[i];
        }
        child
    }

    pub fn get_genome(&self) -> Genome {
        self.genome
    }
}

const TOUR_SIZE: usize = 5;

// Returns the best individual
pub fn elitiste_new_population(
    population: &mut [DNA],
    new_population: &mut [DNA],
    elite_count: usize,
    crossover_rate: &Bernoulli,
    mutation_rate: &Bernoulli,
    game: &Game,
) -> DNA {
    let mut rng = rng();

    let n = population.len();
    // Need to sort using mutable references
    let indices: Vec<usize> = (0..n)
        .sorted_by(|&i, &j| {
            population[j]
                .fitness(game)
                .partial_cmp(&population[i].fitness(game))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .collect();

    let best = population[indices[0]];

    // 1) copy elites deterministically
    for i in 0..elite_count {
        new_population[i] = population[indices[i]];
    }

    let upper_bound = if elite_count == 0 { n } else { elite_count };

    // helper: tournament select one parent
    let tournament = |rng: &mut ThreadRng| -> usize {
        indices[(0..TOUR_SIZE)
            .map(|_| rng.random_range(0..upper_bound))
            .min()
            .unwrap()]
    };

    // 2) fill out the rest
    new_population
        .iter_mut()
        .take(n)
        .skip(elite_count)
        .for_each(|new| {
            let p1_idx = tournament(&mut rng);
            let p1 = &population[p1_idx];
            if crossover_rate.sample(&mut rng) {
                let p2_idx = tournament(&mut rng);
                let p2 = &population[p2_idx];
                *new = p1.crossover(p2);
            } else {
                *new = *p1;
            }
            *new = new.mutate(mutation_rate)
        });
    best
}

pub fn run_simulation(game: &Game, starship: &Starship, params: &SimulationParams) -> Option<DNA> {
    let mut population = (0..params.pop_size)
        .map(|_| {
            let genome = gen_init_rand();
            DNA::new(genome, starship.copy())
        })
        .collect_vec();

    let mut new_population = population.clone();

    let mut first_ok = -1;
    let mut overall_best = Option::<DNA>::None;

    let elite_count = (params.elite_rate * params.pop_size as f64).floor() as usize;
    let crossover_rate = Bernoulli::new(params.crossover_rate).unwrap();
    let mutation_rate = Bernoulli::new(params.mutation_rate).unwrap();

    for generation in 0..params.nb_generations {
        let mut best_individual = elitiste_new_population(
            &mut population,
            &mut new_population,
            elite_count,
            &crossover_rate,
            &mutation_rate,
            game,
        );

        let best_fitness = best_individual.fitness(game);
        if best_fitness >= WINNING_FITNESS {
            if first_ok == -1 {
                first_ok = generation + 1;
            }
            let should_replace = match overall_best.as_mut() {
                None => true,
                Some(best) => best_fitness > best.fitness(game),
            };
            if should_replace {
                overall_best = Some(best_individual);
            }
        }
        std::mem::swap(&mut population, &mut new_population);
    }
    overall_best
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[allow(dead_code)]
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, usize); // the number of points used to draw the surface of Mars.
    let mut game = Game::new(n);
    let points = (1..=n)
        .flat_map(|_| {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            input_line
                .split_whitespace()
                .map(|x| parse_input!(x, usize))
                .collect_vec()
        })
        .collect_vec();

    for i in (0..points.len()).step_by(2) {
        game.add_point(points[i], points[i + 1]);
    }

    let params = SimulationParams {
        pop_size: 200,
        nb_generations: 200,
        crossover_rate: 0.96,
        mutation_rate: 0.055,
        elite_rate: 0.06,
    };

    let mut genome = Option::<Genome>::None;
    let mut i = 0;
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let hs = parse_input!(inputs[2], f32); // the horizontal speed (in m/s), can be negative.
        let vs = parse_input!(inputs[3], f32); // the vertical speed (in m/s), can be negative.
        let f = parse_input!(inputs[4], u16); // the quantity of remaining fuel in liters.
        let r = parse_input!(inputs[5], i8); // the rotation angle in degrees (-90 to 90).
        let p = parse_input!(inputs[6], u8); // the thrust power (0 to 4).

        let mut starship = Starship::new(x, y, f, r, p, hs, vs);

        if genome.is_none() {
            let best = run_simulation(&game, &starship, &params).unwrap();
            genome = Some(best.get_genome());
        }
        let rotate = get_rotate_on_turn(&genome.unwrap(), i);
        let power = get_power_on_turn(&genome.unwrap(), i);
        starship.add_power(power as i16);
        starship.add_rotation(rotate);
        starship.apply_movement();

        // R P. R is the desired rotation angle. P is the desired thrust power.
        println!("{} {}", starship.get_rotation(), starship.get_power());
        i += 1;
    }
}
