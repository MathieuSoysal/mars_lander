use crate::genetics::pheno::Phenotype;

pub mod crossovers;
pub mod elitiste;
pub mod populations;
pub mod selectors;
pub mod statistics;
pub mod roulette;

pub fn random_new_population<T: Phenotype<i32>>(population: &[T], new_population: &mut [T]) {
    for i in 0..population.len() {
        let parent_1 = &population[rand::random::<usize>() % population.len()];
        let parent_2 = &population[rand::random::<usize>() % population.len()];
        new_population[i] = parent_1.crossover(&parent_2);
    }
}
