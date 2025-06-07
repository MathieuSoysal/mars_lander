use rand::Rng;

use crate::{genetics::pheno::Phenotype};

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
            ((rand::random::<u8>() % 3) << GEN_ROTATE_SIZE_BITS) | (rand::random::<u8>() % 3);
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
pub struct DNA<'a> {
    genome: Genome,
    game: &'a Game,
    starship: Starship,
    fitness: i32,
}

impl<'a> DNA<'a> {
    pub fn new(genome: Genome, game: &'a Game, starship: Starship) -> Self {
        DNA {
            genome,
            game,
            starship,
            fitness: -1,
        }
    }

    pub fn get_game(&self) -> &Game {
        self.game
    }

    pub fn to_svg(&self) -> String {
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
            if self.game.starship_is_landing(&s) {
                str.push_str(&format!(r#"" fill="none" stroke="green" stroke-width="{}" />"#,10));
                str.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="{}" fill="none" stroke="green" stroke-width="1" />"#,
                    s.get_x(), HEIGHT as i32 - s.get_y(), 10 + s.get_fuel() as u32 / 200 
                ));
                return str;
            }
            if self.game.starship_is_crash(&s) {
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

pub const WINNING_FITNESS: i32 = 7000 * 500 + 90 * 500 + 500 * 500 + 500 * 500;

#[inline(always)]
fn calc_fitness(land_dist: i32, rot: i32, x_speed: i32, y_speed: i32, fuel: i32) -> i32 {
    (7000 - land_dist) * 500 + (90 - rot.abs()) * 500
        + (500 - x_speed.abs()) * 500 + (500 - y_speed.abs()) * 500 + fuel * 5000
}
/*
TODO Adjust weights
Order of importance:
 - land_distance (if we take account of y, put 8000 instead of 7000)
 - rotation
 - x & y speed
 - fuel (take account of it only for thoose who succeed)
*/
impl<'a> Phenotype<i32> for DNA<'a> {
    fn fitness(&mut self) -> i32 {
        if self.fitness != -1 {
            return self.fitness;
        }
        self.fitness = 0;
        let mut s = self.starship.copy();
        for i in 0..GENOME_SIZE {
            let rotate = get_rotate_on_turn(&self.genome, i);
            let power = get_power_on_turn(&self.genome, i);
            s.add_power(power as i16);
            s.add_rotation(rotate);
            s.apply_movement();

            if self.game.starship_is_landing(&s) {
                self.fitness = calc_fitness(0, 0, 0, 0, s.get_fuel() as i32);
                break;
            }

            if self.game.starship_is_crash(&s) {
                let land_distance = self.game.get_distance_to_landing(&s);

                if land_distance == 0 {
                    self.fitness = calc_fitness(0, s.get_rotation() as i32, s.get_x_speed() as i32, s.get_y_speed() as i32, 0);
                } else {
                    self.fitness = calc_fitness(land_distance, 90, 500, 500, 0);
                }
                break;
            }
        }
        self.fitness
    }

    fn mutate(&self, mutation_rate: f64) -> DNA<'a> {
        let mut mutated = *self;
        mutated.fitness = -1;
        for i in 0..GENOME_SIZE {
            if rand::thread_rng().gen_bool(mutation_rate) {
                mutated.genome[i] = rand::random::<u8>();
            }
        }
        mutated
    }

    fn crossover(&self, other: &Self) -> Self {
        let mut child = *self;
        child.fitness = -1;
        let crossover_point = rand::random::<usize>() % GENOME_SIZE;
        for i in crossover_point..GENOME_SIZE {
            child.genome[i] = other.genome[i];
        }
        child
    }
}

pub fn population_to_svg(population: &[DNA]) -> String {
    let mut svg = String::new();
    svg.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" width="7000" height="3000" viewBox="0 0 7000 3000">"#);
    svg.push_str(r#"<rect width="100%" height="100%" fill="black" />"#);
    svg.push_str(&population[0].get_game().to_svg());
    svg.push_str("<g>\n");
    for dna in population {
        svg.push_str(&dna.to_svg());
    }
    svg.push_str("</g>\n");
    svg.push_str("</svg>");
    svg
}
