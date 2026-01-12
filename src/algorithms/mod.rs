use crate::entities::genome::DNA;

pub mod elitiste;
pub mod roulette;

pub fn random_new_population(population: &[DNA], new_population: &mut [DNA]) {
    new_population
        .iter_mut()
        .take(population.len())
        .for_each(|individual| {
            let parent_1 = &population[rand::random::<usize>() % population.len()];
            let parent_2 = &population[rand::random::<usize>() % population.len()];
            *individual = parent_1.crossover(parent_2);
        });
}
