use std::ops::Div as _;
use rand::{distributions::Bernoulli, prelude::*};

use super::{
    game::{Game, HEIGHT},
    starship::Starship,
};

const GEN_POWER_SIZE_BITS: usize = 2;
const GEN_ROTATE_SIZE_BITS: usize = 2;
const GEN_MASK_POWER: u8 = (1 << GEN_POWER_SIZE_BITS) - 1;
const GEN_MASK_ROTATE: u8 = (1 << GEN_ROTATE_SIZE_BITS) - 1;

pub const GENOME_SIZE: usize = 100;

type Nucleotide = u8;
pub type Genome = [Nucleotide; GENOME_SIZE];

pub fn gen_init_rand() -> Genome {
    let mut genome = [0; GENOME_SIZE];
    for i in 0..GENOME_SIZE {
        genome[i] =
            ((random::<u8>() % 3) << GEN_ROTATE_SIZE_BITS) | (random::<u8>() % 3);
    }
    genome
}

pub fn gen_init_full() -> Genome {
    let genome = [2 << 2 | 2; GENOME_SIZE];
    genome
}

pub fn gen_init_semi_full() -> Genome {
    let genome = [2 << 2 | 1; GENOME_SIZE];
    genome
}

pub fn get_rotate_on_turn(genome: &Genome, nb_turn: usize) -> i8 {
    let turn = genome[nb_turn];
    let rotate = turn & GEN_MASK_ROTATE as u8;
    match rotate {
        0 => 0,
        1 => -15,
        2 => 15,
        _ => 0,
    }
}

pub fn get_power_on_turn(genome: &Genome, nb_turn: usize) -> i8 {
    let turn = genome[nb_turn];
    let power = (turn >> GEN_ROTATE_SIZE_BITS) & GEN_MASK_POWER as u8;
    match power {
        0 => 0,
        1 => -1,
        2 => 1,
        _ => 0,
    }
}

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

    pub fn fuel_left(&self, game: &Game) -> u16 {
        let mut s = self.starship.copy();
        for i in 0..GENOME_SIZE {
            let rotate = get_rotate_on_turn(&self.genome, i);
            let power = get_power_on_turn(&self.genome, i);
            s.add_power(power as i16);
            s.add_rotation(rotate);
            s.apply_movement();

            if game.starship_is_landing(&s) || game.starship_is_crash(&s) {
                break;
            }
        }
        s.get_fuel()
    }

    pub fn to_svg(&self, game: &Game) -> String {
        let mut str = String::new();
        str.push_str(&format!(
            r#"<polyline points="{},{} "#,
            self.starship.get_x(),
            HEIGHT as i32 - self.starship.get_y()
        ));
        let mut s = self.starship.copy();
        for i in 0..GENOME_SIZE {
            let rotate = get_rotate_on_turn(&self.genome, i);
            let power = get_power_on_turn(&self.genome, i);
            s.add_power(power as i16);
            s.add_rotation(rotate);
            s.apply_movement();
            str.push_str(&format!("{},{} ", s.get_x(), HEIGHT as i32 - s.get_y()));
            if game.starship_is_landing(&s) {
                str.push_str(&format!(r#"" fill="none" stroke="green" stroke-width="{}" />"#,10));
                str.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="{}" fill="none" stroke="green" stroke-width="1" />"#,
                    s.get_x(), HEIGHT as i32 - s.get_y(), 10 + s.get_fuel() as u32 / 200 
                ));
                return str;
            }
            if game.starship_is_crash(&s) {
                str.push_str(r#"" fill="none" stroke="white" />"#);
                str.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="{}" fill="none" stroke="white" stroke-width="1" />"#,
                    s.get_x(), HEIGHT as i32 - s.get_y(), 3 
                ));
                return str   ;
            }
        }
        str.push_str(r#"" fill="none" stroke="white" />"#);
        str
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

pub const WINNING_FITNESS: f64 = LAND_DISTANCE_X_WEIGHT + ROTATION_WEIGHT + X_SPEED_WEIGHT + Y_SPEED_WEIGHT;

#[inline(always)]
fn calc_fit(land_dist_x: i32, land_dist_y: i32, rot: i8, x_speed: f32, y_speed: f32, fuel: u16) -> f64 {
    (7000.0 - land_dist_x as f64).div(7000.) * LAND_DISTANCE_X_WEIGHT + 
    (3000.0 - land_dist_y as f64).div(3000.) * LAND_DISTANCE_Y_WEIGHT + 
    (90.0 - rot.abs() as f64).div(90.) * ROTATION_WEIGHT +
    (500.0 - x_speed.abs() as f64).div(500.) * X_SPEED_WEIGHT +
    (500.0 - y_speed.abs() as f64).div(500.) * Y_SPEED_WEIGHT +
    (fuel as f64).div(2000.) * FUEL_WEIGHT
}

impl DNA {
    pub fn fitness(&mut self, game: &Game) -> f64 {
        if self.fitness != -1.0 {
            return self.fitness;
        }
        self.fitness = 0.0;
        let mut s = self.starship.copy();
        for i in 0..GENOME_SIZE {
            let rotate = get_rotate_on_turn(&self.genome, i);
            let power = get_power_on_turn(&self.genome, i);
            s.add_power(power as i16);
            s.add_rotation(rotate);
            s.apply_movement();

            if game.starship_is_landing(&s) {
                self.fitness = calc_fit(0, 0, 0, 0., 0., s.get_fuel());
                break;
            }

            if game.starship_is_crash(&s) {
                let (land_dist_x, land_dist_y) = game.get_distance_to_landing(&s);
                self.fitness = calc_fit(land_dist_x, land_dist_y, s.get_rotation(), s.get_x_speed(), s.get_y_speed(), 0);
                break;
            }
        }
        self.fitness
    }

    pub fn mutate(&self, mutation_rate: &Bernoulli) -> DNA {
        let mut mutated = *self;
        mutated.fitness = -1.;
        for i in 0..GENOME_SIZE {
            if mutation_rate.sample(&mut thread_rng()) {
                mutated.genome[i] = random::<u8>();
            }
        }
        mutated
    }

    pub fn crossover(&self, other: &Self) -> Self {
        let mut child = *self;
        child.fitness = -1.;
        let crossover_point = random::<usize>() % GENOME_SIZE;
        for i in crossover_point..GENOME_SIZE {
            child.genome[i] = other.genome[i];
        }
        child
    }
}

pub fn population_to_svg(population: &[DNA], game: &Game) -> String {
    let mut svg = String::new();
    svg.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" width="7000" height="3000" viewBox="0 0 7000 3000">"#);
    svg.push_str(r#"<rect width="100%" height="100%" fill="black" />"#);
    svg.push_str(&game.to_svg());
    svg.push_str("<g>\n");
    for dna in population {
        svg.push_str(&dna.to_svg(game));
    }
    svg.push_str("</g>\n");
    svg.push_str("</svg>");
    svg
}
