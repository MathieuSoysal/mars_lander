pub mod my_genetics;

use once_cell::sync::Lazy;
use std::array;
use std::sync::Mutex;

use entities::{
    game::Game,
    genome::{DNA, gen_init_rand, population_to_svg},
    starship::Starship,
};
use my_genetics::elitiste::elitiste_new_population;
use wasm_bindgen::prelude::*;

pub mod entities;
pub mod genetics;

const POP_SIZE: usize = 60;

// Lazy static mutable f64 variable that can be accessed and modified from main
pub static SIMULATION_PARAMETER: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(0.5));

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn tes(nb_generations: i32, crossover_rate: f64, mutation_rate: f32) -> Vec<String> {
    log(&format!(
        "{} {} {} ",
        nb_generations, crossover_rate, mutation_rate
    ));
    *SIMULATION_PARAMETER.lock().unwrap() = mutation_rate;

    let mut returned: Vec<String> = Vec::new();
    let mut game = Game::new(10);
    let starship = Starship::new(2500, 2700, 5500, 0, 0, 0., 0.);

    game.add_point(0, 100);
    game.add_point(1000, 500);
    game.add_point(1500, 1500);
    game.add_point(3000, 1000);
    game.add_point(4000, 150);
    game.add_point(5500, 150);
    game.add_point(6999, 800);

    let mut population: [DNA; POP_SIZE] = array::from_fn(|_| {
        let genome = gen_init_rand();
        DNA::new(genome, &game, starship.copy())
    });

    let mut new_population: [DNA; POP_SIZE] = population.clone();

    for _ in 0..nb_generations {
        elitiste_new_population(&mut population, &mut new_population, 20, crossover_rate);
        returned.push(format!("{:?}", population_to_svg(&population)));
        population = new_population;
    }
    returned
}
