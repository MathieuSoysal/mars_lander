
use super::*;
use pheno::{Fitness, Phenotype};
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct StochasticSelector {
    count: usize,
}

impl StochasticSelector {
    pub fn new(count: usize) -> StochasticSelector {
        StochasticSelector { count }
    }
}

impl<T, F> Selector<T, F> for StochasticSelector
where
    T: Phenotype<F>,
    F: Fitness,
{
    fn select<'a>(&self, population: &'a [T]) -> Result<Parents<&'a T>, String> {
        if self.count == 0 || self.count % 2 != 0 || self.count >= population.len() {
            return Err(format!(
                "Invalid parameter `count`: {}. Should be larger than zero, a \
                 multiple of two and less than the population size.",
                self.count
            ));
        }

        let ratio = population.len() / self.count;
        let mut result: Parents<&T> = Vec::new();
        let mut i = ::rand::thread_rng().gen_range(0..population.len());
        let mut selected = 0;
        while selected < self.count {
            result.push((
                &population[i],
                &population[(i + ratio - 1) % population.len()],
            ));
            i += ratio - 1;
            i %= population.len();
            selected += 2;
        }
        Ok(result)
    }
}
