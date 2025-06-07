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
    use colored::*;
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

        // Calculate statistics
        let failed = res.iter().filter(|&&x| x == -1).count();
        let successful: Vec<i32> = res.iter().filter(|&&x| x != -1).cloned().collect();
        let success_rate = (N_TESTS - failed) as f64 * 100.0 / N_TESTS as f64;

        println!("\n{}", "=".repeat(60).bright_cyan());

        // Overall results
        println!("\n{}", "📊 OVERALL RESULTS:".bright_green().bold());
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
                "  Average: {:.2} generations",
                format!("{:.2}", avg_gen).bright_blue().bold()
            );
            println!(
                "  Median: {:.2} generations",
                format!("{:.2}", median_gen).bright_purple().bold()
            );
            println!("  Std Dev: {:.2}", format!("{:.2}", std_dev).bright_white());

            // Performance quality assessment
            println!("\n{}", "🎯 PERFORMANCE ASSESSMENT:".bright_magenta().bold());
            if success_rate >= 95.0 {
                println!("  Quality: {}", "EXCELLENT 🌟".bright_green().bold());
            } else if success_rate >= 70.0 {
                println!("  Quality: {}", "GOOD ✅".bright_blue().bold());
            } else if success_rate >= 50.0 {
                println!("  Quality: {}", "FAIR ⚠️".bright_yellow().bold());
            } else {
                println!("  Quality: {}", "POOR ❌".bright_red().bold());
            }

            if avg_gen <= 10.0 {
                println!("  Speed: {}", "VERY FAST ⚡".bright_green().bold());
            } else if avg_gen <= 20.0 {
                println!("  Speed: {}", "FAST 🏃".bright_blue().bold());
            } else if avg_gen <= 35.0 {
                println!("  Speed: {}", "MODERATE 🚶".bright_yellow().bold());
            } else {
                println!("  Speed: {}", "SLOW 🐌".bright_red().bold());
            }
        }

        println!("{}", "=".repeat(60).bright_cyan());
    }
}
