use crate::genetics::pheno::Phenotype;

use super::{
    game::{self, Game},
    starship::{self, Starship},
};

const GEN_POWER_SIZE_BITS: usize = 2;
const GEN_ROTATE_SIZE_BITS: usize = 2;

type Nucleotide = u8;
pub type Genome = [Nucleotide; 80];

pub fn gen_init_rand() -> Genome {
    let mut genome = [0; 80];
    for i in 0..80 {
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

impl<'a> Phenotype<i32> for DNA<'a> {
    fn fitness(&self) -> i32 {
        let mut starship = self.starship.copy();
        for i in 0..80 {
            let rotate = get_rotate_on_turn(&self.genome, i);
            let power = get_power_on_turn(&self.genome, i);
            starship.add_power(power as i32);
            starship.add_rotation(rotate);
            starship.apply_movement();
            if self.game.starship_is_crash(&starship) {
                return self.game.get_distance_to_landing(&starship) / 10;
            }
            if self.game.starship_is_landing(&starship) {
                return starship.get_fuel() as i32;
            }
        }
        self.game.get_distance_to_landing(&starship) / 10
        }

    fn mutate(&self) -> DNA<'a> {
        let mut mutated = *self;
        for i in 0..80 {
            if rand::random::<bool>() {
                mutated.genome[i] = rand::random::<u8>();
            }
        }
        mutated
    }

    fn crossover(&self, other: &Self) -> Self {
        let mut child = *self;
        let crossover_point = rand::random::<usize>() % 80;
        for i in crossover_point..80 {
            child.genome[i] = other.genome[i];
        }
        child
    }
}

