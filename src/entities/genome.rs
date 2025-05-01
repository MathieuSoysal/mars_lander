use std::io::Write;

use chrono::format;

use crate::genetics::pheno::Phenotype;

use super::{
    game::{self, Game},
    starship::{self, Starship},
};

const GEN_POWER_SIZE_BITS: usize = 2;
const GEN_ROTATE_SIZE_BITS: usize = 2;
const GEN_MASK_POWER: u8 = (1 << GEN_POWER_SIZE_BITS) - 1;
const GEN_MASK_ROTATE: u8 = (1 << GEN_ROTATE_SIZE_BITS) - 1;

const GENOME_SIZE: usize = 100;

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
    let mut genome = [0; GENOME_SIZE];
    for i in 0..GENOME_SIZE {
        genome[i] = 2 << 2 | 2;
    }
    genome
}

pub fn gen_init_semi_full() -> Genome {
    let mut genome = [0; GENOME_SIZE];
    for i in 0..GENOME_SIZE {
        genome[i] = 2 << 2 | 1;
    }
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

    pub fn get_game(&self) -> &Game {
        self.game
    }

    pub fn to_svg(&self) -> String {
        let mut str = String::new();
        str.push_str("<polyline points=\"");
        str.push_str(&format!(
            "{},{} ",
            self.starship.get_x(),
            3000 - self.starship.get_y()
        ));
        let mut s = self.starship.copy();
        for i in 0..GENOME_SIZE {
            let rotate = get_rotate_on_turn(&self.genome, i);
            let power = get_power_on_turn(&self.genome, i);
            s.add_power(power as i32);
            s.add_rotation(rotate);
            s.apply_movement();
            str.push_str(&format!("{},{} ", s.get_x(), 3000 - s.get_y()));
            if self.game.starship_is_landing(&s) {
                str.push_str("\" fill=\"none\" stroke=\"green\" stroke-width=\"80\" />");
                return str;
            }
            if self.game.starship_is_crash(&s) {
                str.push_str("\" fill=\"none\" stroke=\"white\" />");
                str.push_str(&format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"white\" stroke-width=\"1\" />",
                    s.get_x(), 3000 - s.get_y(), 90 - s.get_rotation().abs() as u32 
                ));
                return str;
            }
        }
        str.push_str("\" fill=\"none\" stroke=\"white\" />");
        str
    }
}

impl<'a> Phenotype<i32> for DNA<'a> {
    fn fitness(&self) -> i32 {
        let mut s = self.starship.copy();
        for i in 0..GENOME_SIZE {
            let rotate = get_rotate_on_turn(&self.genome, i);
            let power = get_power_on_turn(&self.genome, i);
            s.add_power(power as i32);
            s.add_rotation(rotate);
            s.apply_movement();
            if self.game.starship_is_landing(&s) {
                return 14000 ;
            }
            if self.game.starship_is_crash(&s) {
                if self.game.get_distance_to_landing(&s) == 0 {
                    return 7000 ;
                }
                return (7000 - self.game.get_distance_to_landing(&s)) / self.game.get_distance_to_landing(&s);
            }
        }
        0
    }

    fn mutate(&self) -> DNA<'a> {
        let mut mutated = *self;
        for i in 0..GENOME_SIZE {
            if rand::random::<f32>() <= 0.1 {
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

pub fn population_to_svg(population: &[DNA], nb: usize) -> String {
    let mut svg = String::new();
    let mut file = std::fs::File::create(format!("all_svg/output{nb}.svg")).unwrap();
    svg.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" width="7000" height="3000" viewBox="0 0 7000 3000">"#);
    svg.push_str(r#"<rect width="100%" height="100%" fill="black" />"#);
    svg.push_str(&population[0].get_game().to_svg());
    svg.push_str("<g>\n");
    for dna in population {
        svg.push_str(&dna.to_svg());
    }
    svg.push_str("</g>\n");
    svg.push_str(r#"</svg>"#);
    file.write_all(svg.as_bytes()).unwrap();
    svg
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::game::Game;
    use crate::entities::starship::Starship;
    use crate::genetics::sim::select::{StochasticSelector, UnstableMaximizeSelector};
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

        let starship = Starship::new(2500, 2700, 1000, 0, 0, 0., 0.);

        let mut population: Vec<DNA> = Vec::with_capacity(400);
        let mut rng = ::rand::thread_rng();
        for _ in 0..300 {
            let genome = gen_init_rand();
            let dna = DNA::new(genome, &game, starship.copy());
            population.push(dna);
        }
        #[allow(deprecated)]
        let mut builder = Simulator::builder(&mut population);
        builder
            .with_selector(Box::new(StochasticSelector::new(50)))
            .with_max_iters(100);
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
