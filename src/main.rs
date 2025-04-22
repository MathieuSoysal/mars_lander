use std::array;
use std::io::Write;

use my_lib::{
    entities::{
        game::Game,
        genome::{gen_init_full, gen_init_rand, gen_init_semi_full, population_to_svg, DNA},
        starship::Starship,
    },
    genetics::{
        pheno::Phenotype,
        sim::{select::UnstableMaximizeSelector, seq::Simulator, Builder, Simulation},
    }, my_genetics::{elitiste::elitiste_new_population, random_new_population, roulette::roulette_new_population},
};

extern crate my_lib;

fn main() {
    let mut game = Game::new(10);
    game.add_point(0, 1500);
    game.add_point(1000, 2000);
    game.add_point(2000, 500);
    game.add_point(3500, 500);
    game.add_point(5000, 1500);
    game.add_point(6999, 1000);

    let starship = Starship::new(3000, 2800, 500, 0, 0, 0., 0.);

    let mut population: [DNA; 100] = array::from_fn(|_| {
        let genome = gen_init_rand();
        DNA::new(genome, &game, starship.copy())
    });
    let mut new_population: [DNA; 100] = array::from_fn(|_| {
        let genome = gen_init_rand();
        DNA::new(genome, &game, starship.copy())
    });
    population[0] = DNA::new(gen_init_full(), &game, starship.copy());
    population[1] = DNA::new(gen_init_semi_full(), &game, starship.copy());
    population_to_svg(&population, 0);
    for i in 1..100 {
        roulette_new_population(&population, &mut new_population, 0.2);
        population_to_svg(&new_population, i);  
        population = new_population;
    }
}
