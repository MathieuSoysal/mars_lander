use crate::genetics::pheno::Phenotype;
use rand::prelude::*;  // thread_rng, Rng

pub fn elitiste_new_population<T>(
    population: &[T],
    new_population: &mut [T],
    elite_count: usize,
    crossover_rate: f64,
)
where
    T: Phenotype<i32> + Clone,
{
    let mut rng = thread_rng();

    let mut indices: Vec<usize> = (0..population.len()).collect();
    indices.sort_by_key(|&i| -population[i].fitness());
    
    // 1) copy elites deterministically
    for (dst, &src_idx) in new_population.iter_mut().zip(indices.iter()).take(elite_count) {
        *dst = population[src_idx].clone();
    }

    // helper: tournament select one parent
    let mut tournament = |rng: &mut ThreadRng| -> &T {
        const TOUR_SIZE: usize = 5;
        let mut best = &population[indices[rng.gen_range(0..elite_count)]];
        for _ in 1..TOUR_SIZE {
            let contender = &population[indices[rng.gen_range(0..elite_count)]];
            if contender.fitness() > best.fitness() {
                best = contender;
            }
        }
        best
    };

    // 2) fill out the rest
    for i in elite_count..population.len() {
        let p1 = tournament(&mut rng);
        if rng.gen_bool(crossover_rate) {
            let p2 = tournament(&mut rng);
            new_population[i] = p1.crossover(p2).mutate();
        } else {
            new_population[i] = p1.clone().mutate();
        }
    }
}
