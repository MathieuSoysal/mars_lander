pub mod algorithms;
mod cg_main; // So that rust-analyzer keep analyzing
pub mod params;

use algorithms::elitiste::elitiste_new_population;
use entities::{
    game::Game,
    genome::{DNA, WINNING_FITNESS, gen_init_rand, population_to_svg},
    starship::Starship,
};
use itertools::Itertools;
use params::SimulationParams;
use rand::distributions::Bernoulli;
use wasm_bindgen::prelude::*;

pub mod entities;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn run_from_web(
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

    let input = map.split('\n').collect_vec();
    let n_points = parse_input!(input[0], usize);
    let points = (1..=n_points)
        .flat_map(|line| {
            input[line]
                .split_whitespace()
                .map(|x| parse_input!(x, usize))
        })
        .collect_vec();

    let (x, y, h_speed, v_speed, fuel, rotate, power) = input[n_points + 1]
        .split_whitespace()
        .map(|x| parse_input!(x, i32))
        .collect_tuple()
        .unwrap();
    let starship = Starship::new(
        x,
        y,
        fuel as u16,
        rotate as i8,
        power as u8,
        h_speed as f32,
        v_speed as f32,
    );

    let mut game = Game::new(n_points);

    for i in (0..points.len()).step_by(2) {
        game.add_point(points[i], points[i + 1]);
    }

    log(&format!("Running simulation with {:?}", params));

    run_simulation(&game, &starship, &params)
}

pub fn run_simulation(game: &Game, starship: &Starship, params: &SimulationParams) -> Vec<String> {
    let mut returned: Vec<String> = Vec::new();

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
    let crossover_rate = Bernoulli::new(params.crossover_rate).unwrap();
    let mutation_rate = Bernoulli::new(params.mutation_rate).unwrap();

    for generation in 0..params.nb_generations {
        let mut best_individual = elitiste_new_population(
            &mut population,
            &mut new_population,
            elite_count,
            &crossover_rate,
            &mutation_rate,
            &game,
        );

        let best_fitness = best_individual.fitness(&game);
        if best_fitness >= WINNING_FITNESS {
            if first_ok == -1 {
                first_ok = generation + 1;
            }
            let should_replace = match overall_best.as_mut() {
                None => true,
                Some(best) => best_fitness > best.fitness(&game),
            };
            if should_replace {
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
    use super::*;
    use crate::entities::genome::{
        LAND_DISTANCE_X_WEIGHT, LAND_DISTANCE_Y_WEIGHT, ROTATION_WEIGHT, WINNING_FITNESS,
        X_SPEED_WEIGHT, Y_SPEED_WEIGHT,
    };
    use colored::*;
    use rand::distributions::Bernoulli;
    use rayon::prelude::*;
    use std::env;

    const MINIMAL_STATS: bool = true;
    const N_TESTS_ALL: usize = 3000;
    const N_TESTS_MAP: usize = 6000;
    const PARAMS: SimulationParams = SimulationParams {
        pop_size: 100,
        nb_generations: 50,
        crossover_rate: 0.96,
        mutation_rate: 0.045,
        elite_rate: 0.06,
    };

    const DEFAULT_MAP: &str = "7
    0 100
    1000 500
    1500 1500
    3000 1000
    4000 150
    5500 150
    6999 800
    2500 2700 0 0 550 0 0";
    const CANYON_MAP: &str = "10
    0 100
    1000 500
    1500 100
    3000 100
    3500 500
    3700 200
    5000 1500
    5800 300
    6000 1000
    6999 2000
    6500 2800 -100 0 600 90 0";
    const MOUNTAIN_MAP: &str = "7
    0 100
    1000 500
    1500 1500
    3000 1000
    4000 150
    5500 150
    6999 800
    6500 2800 -90 0 750 90 0";
    const PLATEAU_MAP: &str = "20
    0 1000
    300 1500
    350 1400
    500 2000
    800 1800
    1000 2500
    1200 2100
    1500 2400
    2000 1000
    2200 500
    2500 100
    2900 800
    3000 500
    3200 1000
    3500 2000
    3800 800
    4000 200
    5000 200
    5500 1500
    6999 2800
    500 2700 100 0 800 -90 0";
    const VALLEY_MAP: &str = "20
    0 1000
    300 1500
    350 1400
    500 2100
    1500 2100
    2000 200
    2500 500
    2900 300
    3000 200
    3200 1000
    3500 500
    3800 800
    4000 200
    4200 800
    4800 600
    5000 1200
    5500 900
    6000 500
    6500 300
    6999 500
    6500 2700 -50 0 1000 90 0";
    // const CAVE_RIGHT: &str = "22
    // 0 450
    // 300 750
    // 1000 450
    // 1500 650
    // 1800 850
    // 2000 1950
    // 2200 1850
    // 2400 2000
    // 3100 1800
    // 3150 1550
    // 2500 1600
    // 2200 1550
    // 2100 750
    // 2200 150
    // 3200 150
    // 3500 450
    // 4000 950
    // 4500 1450
    // 5000 1550
    // 5500 1500
    // 6000 950
    // 6999 1750
    // 6500 2600 -20 0 1000 45 0";
    // const CAVE_LEFT: &str = "18
    // 0 1800
    // 300 1200
    // 1000 1550
    // 2000 1200
    // 2500 1650
    // 3700 220
    // 4700 220
    // 4750 1000
    // 4700 1650
    // 4000 1700
    // 3700 1600
    // 3750 1900
    // 4000 2100
    // 4900 2050
    // 5100 1000
    // 5500 500
    // 6200 800
    // 6999 600
    // 6500 2000 0 0 1200 0 0";

    fn get_maps() -> Vec<(&'static str, &'static str)> {
        vec![
            ("default", DEFAULT_MAP),
            ("canyon", CANYON_MAP),
            ("mountain", MOUNTAIN_MAP),
            ("plateau", PLATEAU_MAP),
            ("valley", VALLEY_MAP),
            // ("cave_right", CAVE_RIGHT),
            // ("cave_left", CAVE_LEFT),
        ]
    }

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
        let crossover_rate = Bernoulli::new(params.crossover_rate).unwrap();
        let mutation_rate = Bernoulli::new(params.mutation_rate).unwrap();

        for generation in 0..params.nb_generations {
            let mut best_individual = elitiste_new_population(
                &mut population,
                &mut new_population,
                elite_count,
                &crossover_rate,
                &mutation_rate,
                game,
            );

            let best_fitness = best_individual.fitness(game);
            if best_fitness >= WINNING_FITNESS {
                if first_ok == -1 {
                    first_ok = generation + 1;
                }
                let should_replace = match overall_best.as_mut() {
                    None => true,
                    Some(best) => best_fitness > best.fitness(game),
                };
                if should_replace {
                    overall_best = Some(best_individual);
                }
            }
            std::mem::swap(&mut population, &mut new_population);
        }
        (first_ok, overall_best)
    }

    fn run_perf_test_for_map(
        map_name: &str,
        map_data: &str,
        params: &SimulationParams,
        n_tests: usize,
        minimal_stats: bool,
    ) {
        if !minimal_stats {
            println!(
                "\n\n{}",
                format!("===== Running tests for map: {} =====", map_name)
                    .to_uppercase()
                    .bold()
                    .yellow()
            );
        }

        let input = map_data.split('\n').collect_vec();
        let n_points = parse_input!(input[0], usize);
        let points = (1..=n_points)
            .flat_map(|line| {
                input[line]
                    .split_whitespace()
                    .map(|x| parse_input!(x, usize))
            })
            .collect_vec();

        let (x, y, h_speed, v_speed, fuel, rotate, power) = input[n_points + 1]
            .split_whitespace()
            .map(|x| parse_input!(x, i32))
            .collect_tuple()
            .unwrap();
        let starship = Starship::new(
            x,
            y,
            fuel as u16,
            rotate as i8,
            power as u8,
            h_speed as f32,
            v_speed as f32,
        );

        let mut game = Game::new(n_points);

        for i in (0..points.len()).step_by(2) {
            game.add_point(points[i], points[i + 1]);
        }

        let res: Vec<(i32, Option<DNA>)> = (0..n_tests)
            .into_par_iter()
            .map(|_| test_one(params, &game, &starship))
            .collect();

        // Calculate statistics
        let failed = res.iter().filter(|&&(x, _)| x == -1).count();
        let mut successfull_bests: Vec<(i32, DNA)> = res
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
            .iter_mut()
            .map(|(_, dna)| fuel as u16 - dna.fuel_left(&game))
            .collect();
        let success_rate = (n_tests - failed) as f64 * 100.0 / n_tests as f64;

        if minimal_stats {
            let avg_gen = if successful.is_empty() {
                -1.0
            } else {
                successful.iter().map(|&x| x as f64).sum::<f64>() / successful.len() as f64
            };
            println!(
                "  {:<10} | Success rate: {:>6.2}% | Avg generations: {:>5.2}",
                map_name.bold(),
                success_rate,
                avg_gen
            );
            return;
        }

        println!("{}", "=".repeat(60).bright_cyan());

        // Overall results
        println!("{}", "📊 OVERALL RESULTS:".bright_green().bold());
        println!(
            "  Total runs: {}",
            n_tests.to_string().bright_white().bold()
        );
        println!(
            "  Successful: {}",
            (n_tests - failed).to_string().bright_green().bold()
        );
        println!("  Failed: {}", failed.to_string().bright_red().bold());
        println!(
            "  Success rate: {}%",
            format!("{:.2}", success_rate).bright_cyan().bold()
        );
        // Helper function for statistics
        fn print_statistics<T, F>(
            data: &[T],
            label: &str,
            unit: &str,
            color: colored::Color,
            value_fn: F,
        ) where
            T: Copy + Ord + std::fmt::Display + Into<f64>,
            F: Fn(&T) -> f64,
        {
            if data.is_empty() {
                return;
            }
            let min = *data.iter().min().unwrap();
            let max = *data.iter().max().unwrap();
            let avg = data.iter().map(|&x| value_fn(&x)).sum::<f64>() / data.len() as f64;

            // Median
            let mut sorted = data.to_vec();
            sorted.sort();
            let median = if sorted.len() % 2 == 0 {
                (value_fn(&sorted[sorted.len() / 2 - 1]) + value_fn(&sorted[sorted.len() / 2]))
                    / 2.0
            } else {
                value_fn(&sorted[sorted.len() / 2])
            };

            // Std dev
            let variance = data
                .iter()
                .map(|&x| {
                    let diff = value_fn(&x) - avg;
                    diff * diff
                })
                .sum::<f64>()
                / data.len() as f64;
            let std_dev = variance.sqrt();

            println!("\n{}", label.bright_green().bold());
            println!("  Min: {} {}", min.to_string().color(color).bold(), unit);
            println!("  Max: {} {}", max.to_string().bright_yellow(), unit);
            println!(
                "  Average: {} {}",
                format!("{:.2}", avg).bright_blue().bold(),
                unit
            );
            println!(
                "  Median: {} {}",
                format!("{:.2}", median).bright_purple().bold(),
                unit
            );
            println!("  Std Dev: {}", format!("{:.2}", std_dev).bright_white());
        }

        print_statistics(
            &successful,
            "📈 SUCCESS STATISTICS (generations to solution):",
            "generations",
            colored::Color::Green,
            |&x| x as f64,
        );

        print_statistics(
            &successful_fuel,
            "⛽ FUEL STATISTICS (fuel used):",
            "units",
            colored::Color::Green,
            |&x| x as f64,
        );

        println!("{}", "=".repeat(60).bright_cyan());
    }

    #[test]
    fn test_perfs_all() {
        rayon::ThreadPoolBuilder::new()
            .num_threads(16)
            .build_global()
            .unwrap();

        let maps = get_maps();

        if MINIMAL_STATS {
            println!(
                "{}",
                "===== Minimalist Performance Stats ====="
                    .to_uppercase()
                    .bold()
                    .green()
            );
            println!(
                "{:?}",
                vec![
                    LAND_DISTANCE_X_WEIGHT,
                    LAND_DISTANCE_Y_WEIGHT,
                    ROTATION_WEIGHT,
                    X_SPEED_WEIGHT,
                    Y_SPEED_WEIGHT
                ]
            );
        }

        for (map_name, map_data) in maps {
            run_perf_test_for_map(map_name, map_data, &PARAMS, N_TESTS_ALL, MINIMAL_STATS);
        }
    }

    #[test]
    fn test_perfs_map() {
        rayon::ThreadPoolBuilder::new()
            .num_threads(16)
            .build_global()
            .unwrap();

        let maps = get_maps();

        let args: Vec<String> = env::args().collect();
        let map_to_run = args.get(3).map(|s| s.to_lowercase());

        let map_to_run = map_to_run.as_deref().unwrap_or("default");

        if let Some((map_name, map_data)) = maps.iter().find(|(name, _)| *name == map_to_run) {
            run_perf_test_for_map(map_name, map_data, &PARAMS, N_TESTS_MAP, MINIMAL_STATS);
        } else {
            println!("Map '{}' not found. Running default map.", map_to_run);
            run_perf_test_for_map(maps[0].0, maps[0].1, &PARAMS, N_TESTS_MAP, MINIMAL_STATS);
        }
    }
}
