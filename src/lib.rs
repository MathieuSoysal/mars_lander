pub mod my_genetics;

use std::array;

use entities::{
    game::Game,
    genome::{gen_init_full, gen_init_rand, gen_init_semi_full, get_power_on_turn, get_rotate_on_turn, population_to_svg, DNA},
    starship::{self, Starship},
};
use my_genetics::elitiste::elitiste_new_population;
use wasm_bindgen::prelude::*;

pub mod entities;
pub mod genetics;


#[wasm_bindgen]
pub fn tes(nb_generations: i32) -> Vec<String> {
    let mut returned : Vec<String> = Vec::new();
    let mut game = Game::new(10);
    let mut starship = Starship::new(6500, 1900, 10000, 0, 0, 0., 0.);
    game.add_point(0, 1500);
    game.add_point(1000, 2000);
    game.add_point(2000, 500);
    game.add_point(3500, 500);
    game.add_point(5000, 1500);
    game.add_point(6999, 1000);

    let mut population: [DNA; 100] = array::from_fn(|_| {
        let genome = gen_init_rand();
        DNA::new(genome, &game, starship.copy())
    });
    let mut new_population: [DNA; 100] = population.clone();
    for _ in 0..nb_generations {
        elitiste_new_population(
            &mut population,
            &mut new_population,
            10,
            0.6,
        );
        returned.push(format!("{:?}", population_to_svg(&population)));
        population = new_population;
    }
    returned
}
