use crate::genetics::pheno::Phenotype;
use rand::prelude::*;  // thread_rng, Rng

pub fn elitiste_new_population<T>(
    population: &mut [T],
    new_population: &mut [T],
    elite_count: usize,
    crossover_rate: f64,
)
where
    T: Phenotype<i32> + Clone,
{
    let mut rng = thread_rng();

    let mut indices: Vec<usize> = (0..population.len()).collect();
    // Need to sort using mutable references
    indices.sort_by_key(|&i| {
        let mut phenotype = population[i].clone();
        -phenotype.fitness()
    });
    
    // 1) copy elites deterministically
    for (dst, &src_idx) in new_population.iter_mut().zip(indices.iter()).take(elite_count) {
        *dst = population[src_idx].clone();
    }

    // helper: tournament select one parent
    let tournament = |rng: &mut ThreadRng| -> &T {
        const TOUR_SIZE: usize = 5;
        let mut best_idx = indices[rng.gen_range(0..elite_count)];
        let mut best_fitness = {
            let mut phenotype = population[best_idx].clone();
            phenotype.fitness()
        };
        
        for _ in 1..TOUR_SIZE {
            let contender_idx = indices[rng.gen_range(0..elite_count)];
            let contender_fitness = {
                let mut phenotype = population[contender_idx].clone();
                phenotype.fitness()
            };
            
            if contender_fitness > best_fitness {
                best_idx = contender_idx;
                best_fitness = contender_fitness;
            }
        }
        &population[best_idx]
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
