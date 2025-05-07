pub mod my_genetics;
pub mod params;

use entities::{
    game::Game,
    genome::{DNA, gen_init_rand, population_to_svg},
    starship::Starship,
};
use itertools::Itertools;
use my_genetics::elitiste::elitiste_new_population;
use params::SimulationParams;
use wasm_bindgen::prelude::*;

pub mod entities;
pub mod genetics;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn run_simulation(
    map: &str,
    pop_size: usize,
    nb_generations: i32,
    crossover_rate: f64,
    mutation_rate: f64,
    elite_rate: f64,
) -> Vec<String> {
    let params = SimulationParams {
        pop_size,
        nb_generations,
        crossover_rate,
        mutation_rate,
        elite_rate,
    };
    params::set_params(params);

    log(&format!(
        "Running simulation with {:?}",
        params::get_params()
    ));

    let mut returned: Vec<String> = Vec::new();
    let mut game = Game::new(10);
    let starship = Starship::new(2500, 2700, 5500, 0, 0, 0., 0.);

    let points = map;

    let points = points.replace(|c: char| !c.is_ascii_digit(), " ");
    let points = points
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    for i in (0..points.len()).step_by(2) {
        game.add_point(points[i], points[i + 1]);
        // log(&format!("Point({}, {})", points[i], points[i + 1]));
    }

    let mut population = (0..params.pop_size)
        .map(|_| {
            let genome = gen_init_rand();
            DNA::new(genome, &game, starship.copy())
        })
        .collect_vec();

    let mut new_population = population.clone();

    let mut first_ok = -1;
    let elite_count = (params.elite_rate * params.pop_size as f64).floor() as usize;
    for generation in 0..params.nb_generations {
        let found_solution = elitiste_new_population(
            &mut population,
            &mut new_population,
            elite_count,
            params.crossover_rate,
        );

        if first_ok == -1 && found_solution {
            first_ok = generation + 1;
        }
        returned.push(population_to_svg(&population));
        std::mem::swap(&mut population, &mut new_population);
    }
    log(&format!("First ok: {}", first_ok));
    returned
}
