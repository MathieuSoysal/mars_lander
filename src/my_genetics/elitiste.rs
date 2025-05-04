use crate::genetics::pheno::Phenotype;
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

    let mut indices: Vec<usize> = (0..population.len()).collect();
    // Need to sort using mutable references
    indices.sort_by_key(|&i| -population[i].fitness());

    let solved = population[indices[0]].fitness() >= WINNING_FITNESS;

    // 1) copy elites deterministically
    for (dst, &src_idx) in new_population
        .iter_mut()
        .zip(indices.iter())
        .take(elite_count)
    {
        *dst = population[src_idx].clone();
    }

    let n = population.len();

    // helper: tournament select one parent
    let mut tournament = |rng: &mut ThreadRng| -> T {
        const TOUR_SIZE: usize = 5;
        let mut best_idx = indices[rng.gen_range(0..elite_count)];
        let mut best_fitness = population[best_idx].fitness();

        for _ in 1..TOUR_SIZE {
            let contender_idx = indices[rng.gen_range(0..elite_count)];
            let contender_fitness = population[contender_idx].fitness();

            if contender_fitness > best_fitness {
                best_idx = contender_idx;
                best_fitness = contender_fitness;
            }
        }
        population[best_idx].clone()
    };

    // 2) fill out the rest
    for i in elite_count..n {
        let p1 = tournament(&mut rng);
        if rng.gen_bool(crossover_rate) {
            let p2 = tournament(&mut rng);
            new_population[i] = p1.crossover(&p2).mutate();
        } else {
            new_population[i] = p1.mutate();
        }
    }
    solved
}
