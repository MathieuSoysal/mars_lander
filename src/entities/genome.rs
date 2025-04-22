use crate::genetics::pheno::Phenotype;

use super::{
    game::{self, Game},
    starship::{self, Starship},
};

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

impl <'a> DNA<'a> {
    pub fn new(genome: Genome, game: &'a Game, starship: Starship) -> Self {
        DNA { genome, game, starship }
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
        let distance = self.game.get_distance_to_landing(&s);
        for i in 0..GENOME_SIZE {
            let rotate = get_rotate_on_turn(&self.genome, i);
            let power = get_power_on_turn(&self.genome, i);
            s.add_power(power as i32);
            s.add_rotation(rotate);
            s.apply_movement();
            if self.game.starship_is_crash(&s) {
                return distance - self.game.get_distance_to_landing(&s);
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

        let starship = Starship::new(2500, 2700, 5500, 0, 0, 0., 0.);

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
