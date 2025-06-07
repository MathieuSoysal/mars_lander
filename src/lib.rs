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

    log(&format!("Running simulation with {:?}", params));

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
            params.mutation_rate,
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

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::prelude::*;

    fn test_one() -> i32 {
        let params = SimulationParams {
            pop_size: 100,
            nb_generations: 50,
            crossover_rate: 0.96,
            mutation_rate: 0.045,
            elite_rate: 0.045,
        };

        let mut game = Game::new(10);
        let starship = Starship::new(2500, 2700, 5500, 0, 0, 0., 0.);

        let points = "(0, 100)
        (1000, 500)
        (1500, 1500)
        (3000, 1000)
        (4000, 150)
        (5500, 150)
        (6999, 800)
        ";

        let points = points.replace(|c: char| !c.is_ascii_digit(), " ");
        let points = points
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();

        for i in (0..points.len()).step_by(2) {
            game.add_point(points[i], points[i + 1]);
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
                params.mutation_rate,
            );

            if first_ok == -1 && found_solution {
                first_ok = generation + 1;
            }
            std::mem::swap(&mut population, &mut new_population);
        }
        first_ok
    }

    #[test]
    fn test_perfs() {
        rayon::ThreadPoolBuilder::new()
            .num_threads(20)
            .build_global()
            .unwrap();

        const N_TESTS: usize = 15000;
        let res: Vec<i32> = (0..N_TESTS).into_par_iter().map(|_| test_one()).collect();
        let failed = res.iter().filter(|&&x| x == -1).count();
        println!(
            "Results: {:?} \n({}/{} failed ({:.1}%))",
            res,
            failed,
            N_TESTS,
            failed as f64 * 100.0 / N_TESTS as f64
        );
    }
}
