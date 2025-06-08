pub mod algorithms;
pub mod params;

use algorithms::elitiste::elitiste_new_population;
use entities::{
    game::Game,
    genome::{DNA, gen_init_rand, population_to_svg},
    starship::Starship,
};
use itertools::Itertools;
use params::SimulationParams;
use wasm_bindgen::prelude::*;

use crate::entities::genome::WINNING_FITNESS;

pub mod entities;

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
            DNA::new(genome, starship.copy())
        })
        .collect_vec();

    let mut new_population = population.clone();

    let mut first_ok = -1;
    let mut overall_best = Option::<DNA>::None;

    let elite_count = (params.elite_rate * params.pop_size as f64).floor() as usize;
    for generation in 0..params.nb_generations {
        let mut best_individual = elitiste_new_population(
            &mut population,
            &mut new_population,
            elite_count,
            params.crossover_rate,
            params.mutation_rate,
            &game,
        );

        if best_individual.fitness(&game) >= WINNING_FITNESS {
            if first_ok == -1 {
                first_ok = generation + 1;
            }
            if overall_best
                .is_none_or(|mut best| best.fitness(&game) < best_individual.fitness(&game))
            {
                overall_best = Some(best_individual);
            }
        }
        returned.push(population_to_svg(&population, &game));
        std::mem::swap(&mut population, &mut new_population);
    }
    log(&format!("First ok: {}", first_ok));
    if let Some(best) = overall_best {
        log(&format!("Best fuel: {}", best.fuel_left(&game)));
    }
    returned
}

#[cfg(test)]
mod tests {
    use crate::entities::genome::WINNING_FITNESS;

    use super::*;
    use colored::*;
    use rayon::prelude::*;

    fn test_one(params: &SimulationParams, game: &Game, starship: &Starship) -> (i32, Option<DNA>) {
        let mut population = (0..params.pop_size)
            .map(|_| {
                let genome = gen_init_rand();
                DNA::new(genome, starship.copy())
            })
            .collect_vec();

        let mut new_population = population.clone();

        let mut first_ok = -1;
        let mut overall_best = Option::<DNA>::None;
        let elite_count = (params.elite_rate * params.pop_size as f64).floor() as usize;
        for generation in 0..params.nb_generations {
            let mut best_individual = elitiste_new_population(
                &mut population,
                &mut new_population,
                elite_count,
                params.crossover_rate,
                params.mutation_rate,
                game,
            );

            if best_individual.fitness(game) >= WINNING_FITNESS {
                if first_ok == -1 {
                    first_ok = generation + 1;
                }
                if overall_best
                    .is_none_or(|mut best| best.fitness(game) < best_individual.fitness(game))
                {
                    overall_best = Some(best_individual);
                }
            }
            std::mem::swap(&mut population, &mut new_population);
        }
        (first_ok, overall_best)
    }

    #[test]
    fn test_perfs() {
        rayon::ThreadPoolBuilder::new()
            .num_threads(16)
            .build_global()
            .unwrap();

        const N_TESTS: usize = 10000;

        let params = SimulationParams {
            pop_size: 100,
            nb_generations: 50,
            crossover_rate: 0.96,
            mutation_rate: 0.055,
            elite_rate: 0.06,
        };

        const INITIAL_FUEL: u16 = 5500;
        let starship = Starship::new(2500, 2700, INITIAL_FUEL, 0, 0, 0., 0.);

        let mut game = Game::new(10);

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

        let res: Vec<(i32, Option<DNA>)> = (0..N_TESTS)
            .into_par_iter()
            .map(|_| test_one(&params, &game, &starship))
            .collect();

        // Calculate statistics
        let failed = res.iter().filter(|&&(x, _)| x == -1).count();
        let successfull_bests: Vec<(i32, DNA)> = res
            .into_iter()
            .filter_map(|(x, dna)| {
                if x != -1 {
                    Some((x, dna.unwrap()))
                } else {
                    None
                }
            })
            .collect();
        let successful: Vec<i32> = successfull_bests.iter().map(|&(x, _)| x).collect();
        let successful_fuel: Vec<u16> = successfull_bests
            .iter()
            .map(|(_, dna)| INITIAL_FUEL - dna.fuel_left(&game))
            .collect();
        let success_rate = (N_TESTS - failed) as f64 * 100.0 / N_TESTS as f64;

        println!("{}", "=".repeat(60).bright_cyan());

        // Overall results
        println!("{}", "📊 OVERALL RESULTS:".bright_green().bold());
        println!(
            "  Total runs: {}",
            N_TESTS.to_string().bright_white().bold()
        );
        println!(
            "  Successful: {}",
            (N_TESTS - failed).to_string().bright_green().bold()
        );
        println!("  Failed: {}", failed.to_string().bright_red().bold());
        println!(
            "  Success rate: {}%",
            format!("{:.2}", success_rate).bright_cyan().bold()
        );

        if !successful.is_empty() {
            // Success statistics
            let min_gen = *successful.iter().min().unwrap();
            let max_gen = *successful.iter().max().unwrap();
            let avg_gen = successful.iter().sum::<i32>() as f64 / successful.len() as f64;

            // Calculate median
            let mut sorted_successful = successful.clone();
            sorted_successful.sort();
            let median_gen = if sorted_successful.len() % 2 == 0 {
                (sorted_successful[sorted_successful.len() / 2 - 1]
                    + sorted_successful[sorted_successful.len() / 2]) as f64
                    / 2.0
            } else {
                sorted_successful[sorted_successful.len() / 2] as f64
            };

            // Calculate standard deviation
            let variance = successful
                .iter()
                .map(|&x| (x as f64 - avg_gen).powi(2))
                .sum::<f64>()
                / successful.len() as f64;
            let std_dev = variance.sqrt();

            println!(
                "\n{}",
                "📈 SUCCESS STATISTICS (generations to solution):"
                    .bright_green()
                    .bold()
            );
            println!(
                "  Fastest: {} generations",
                min_gen.to_string().bright_green().bold()
            );
            println!(
                "  Slowest: {} generations",
                max_gen.to_string().bright_yellow()
            );
            println!(
                "  Average: {} generations",
                format!("{:.2}", avg_gen).bright_blue().bold()
            );
            println!(
                "  Median: {} generations",
                format!("{:.2}", median_gen).bright_purple().bold()
            );
            println!("  Std Dev: {}", format!("{:.2}", std_dev).bright_white());
        }

        if !successful_fuel.is_empty() {
            // Fuel statistics
            let min_fuel = *successful_fuel.iter().min().unwrap();
            let max_fuel = *successful_fuel.iter().max().unwrap();
            let avg_fuel = successful_fuel.iter().map(|&x| x as u32).sum::<u32>() as f64
                / successful_fuel.len() as f64;

            // Calculate median for fuel
            let mut sorted_successful_fuel = successful_fuel.clone();
            sorted_successful_fuel.sort();
            let median_fuel = if sorted_successful_fuel.len() % 2 == 0 {
                (sorted_successful_fuel[sorted_successful_fuel.len() / 2 - 1] as u32
                    + sorted_successful_fuel[sorted_successful_fuel.len() / 2] as u32)
                    as f64
                    / 2.0
            } else {
                sorted_successful_fuel[sorted_successful_fuel.len() / 2] as f64
            };

            // Calculate standard deviation for fuel
            let fuel_variance = successful_fuel
                .iter()
                .map(|&x| (x as f64 - avg_fuel).powi(2))
                .sum::<f64>()
                / successful_fuel.len() as f64;
            let fuel_std_dev = fuel_variance.sqrt();

            println!(
                "\n{}",
                "⛽ FUEL STATISTICS (fuel used):".bright_green().bold()
            );
            println!(
                "  Min Fuel: {} units",
                min_fuel.to_string().bright_green().bold()
            );
            println!("  Max Fuel: {} units", max_fuel.to_string().bright_yellow());
            println!(
                "  Average Fuel: {} units",
                format!("{:.2}", avg_fuel).bright_blue().bold()
            );
            println!(
                "  Median Fuel: {} units",
                format!("{:.2}", median_fuel).bright_purple().bold()
            );
            println!(
                "  Std Dev Fuel: {}",
                format!("{:.2}", fuel_std_dev).bright_white()
            );
        }

        println!("{}", "=".repeat(60).bright_cyan());
    }
}
