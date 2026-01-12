use rand::distributions::{Bernoulli, Uniform};
use rand::{Rng, thread_rng};

use crate::entities::game::Game;
use crate::entities::genome::DNA;

pub fn roulette_new_population(
    population: &[DNA],
    new_population: &mut [DNA],
    crossover_rate: f64,
    game: &Game,
) {
    // 1) Compute (non‐negative) fitnesses
    let fitnesses: Vec<f64> = population
        .iter()
        .map(|ind| ind.clone().fitness(game).max(0.))
        .collect();
    let total_fitness: f64 = fitnesses.iter().sum();
    let mut rng = thread_rng();
    let dist = Uniform::new(0.0, total_fitness);

    // 2) Fill new_population by roulette‐wheel sampling
    for slot in new_population.iter_mut() {
        let mut pick = rng.sample(dist);
        for (ind, &fit) in population.iter().zip(fitnesses.iter()) {
            pick -= fit;
            if pick <= 0.0 {
                *slot = *ind;
                break;
            }
        }
    }

    // 3) Crossover and mutation
    for i in (0..new_population.len()).step_by(2) {
        if rng.gen_bool(crossover_rate) {
            let p1 = new_population[i];
            let p2 = new_population[i + 1];
            new_population[i] = p1.crossover(&p2).mutate(&Bernoulli::new(0.0).unwrap());
            new_population[i + 1] = p2.crossover(&p1);
        }
    }
}
