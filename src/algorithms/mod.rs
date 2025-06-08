use crate::entities::genome::DNA;

pub mod elitiste;
pub mod roulette;

pub fn random_new_population<'a>(population: &[DNA<'a>], new_population: &mut [DNA<'a>]) {
    for i in 0..population.len() {
        let parent_1 = &population[rand::random::<usize>() % population.len()];
        let parent_2 = &population[rand::random::<usize>() % population.len()];
        new_population[i] = parent_1.crossover(&parent_2);
    }
}
