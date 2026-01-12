use crate::entities::{game::Game, genome::DNA};
use itertools::Itertools;
use rand::{distributions::Bernoulli, prelude::*};

const TOUR_SIZE: usize = 5;

// Returns the best individual
pub fn elitiste_new_population(
    population: &mut [DNA],
    new_population: &mut [DNA],
    elite_count: usize,
    crossover_rate: &Bernoulli,
    mutation_rate: &Bernoulli,
    game: &Game,
) -> DNA {
    let mut rng = thread_rng();

    let n = population.len();
    // Need to sort using mutable references
    let indices: Vec<usize> = (0..n)
        .sorted_by(|&i, &j| {
            population[j]
                .fitness(game)
                .partial_cmp(&population[i].fitness(game))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .collect();

    let best = population[indices[0]];

    // 1) copy elites deterministically
    for i in 0..elite_count {
        new_population[i] = population[indices[i]];
    }

    let upper_bound = if elite_count == 0 { n } else { elite_count };

    // helper: tournament select one parent
    let tournament = |rng: &mut ThreadRng| -> usize {
        indices[(0..TOUR_SIZE)
            .map(|_| rng.gen_range(0..upper_bound))
            .min()
            .unwrap()]
    };

    // 2) fill out the rest
    new_population
        .iter_mut()
        .take(n)
        .skip(elite_count)
        .for_each(|new| {
            let p1_idx = tournament(&mut rng);
            let p1 = &population[p1_idx];
            if crossover_rate.sample(&mut rng) {
                let p2_idx = tournament(&mut rng);
                let p2 = &population[p2_idx];
                *new = p1.crossover(p2);
            } else {
                *new = *p1;
            }
            *new = new.mutate(mutation_rate)
        });
    best
}
