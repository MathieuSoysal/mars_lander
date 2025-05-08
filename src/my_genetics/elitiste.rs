use crate::genetics::pheno::Phenotype;
use itertools::Itertools;
use rand::prelude::*;

const WINNING_FITNESS: i32 = 7000 * 500 + 90 * 500 + 90 * 500 + 90 * 500;

// Returns true if any individual solved the problem
pub fn elitiste_new_population<T>(
    population: &mut [T],
    new_population: &mut [T],
    elite_count: usize,
    crossover_rate: f64,
) -> bool
where
    T: Phenotype<i32> + Clone,
{
    let mut rng = thread_rng();

    let n = population.len();
    // Need to sort using mutable references
    let indices: Vec<usize> = (0..n)
        .sorted_by_key(|&i| -population[i].fitness())
        .collect();

    let solved = population[indices[0]].fitness() >= WINNING_FITNESS;

    // 1) copy elites deterministically
    for i in 0..elite_count {
        new_population[i] = population[indices[i]].clone();
    }

    let upper_bound = if elite_count == 0 { n } else { elite_count };

    // helper: tournament select one parent
    let tournament = |rng: &mut ThreadRng| -> usize {
        const TOUR_SIZE: usize = 5;
        indices[(0..TOUR_SIZE)
            .map(|_| rng.gen_range(0..upper_bound))
            .min()
            .unwrap()]
    };

    // 2) fill out the rest
    for i in elite_count..n {
        let p1_idx = tournament(&mut rng);
        let p1 = &population[p1_idx];
        if rng.gen_bool(crossover_rate) {
            let p2_idx = tournament(&mut rng);
            let p2 = &population[p2_idx];
            new_population[i] = p1.crossover(p2).mutate();
        } else {
            new_population[i] = p1.mutate();
        }
    }
    solved
}
