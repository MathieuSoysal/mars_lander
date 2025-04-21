use crate::genetics::pheno;

use super::*;
use pheno::{Fitness, Phenotype};

#[derive(Clone, Copy, Debug)]
pub struct UnstableMaximizeSelector {
    count: usize,
}

impl UnstableMaximizeSelector {
    pub fn new(count: usize) -> UnstableMaximizeSelector {
        UnstableMaximizeSelector { count }
    }
}

impl<T, F> Selector<T, F> for UnstableMaximizeSelector
where
    T: Phenotype<F>,
    F: Fitness,
    T: Send,
    T: Sync,
{
    fn select<'a>(&self, population: &'a [T]) -> Result<Parents<&'a T>, String> {
        if self.count == 0 || self.count % 2 != 0 || self.count * 2 >= population.len() {
            return Err(format!(
                "Invalid parameter `count`: {}. Should be larger than zero, a \
                 multiple of two and less than half the population size.",
                self.count
            ));
        }

        let mut borrowed: Vec<&T> = population.iter().collect();
        borrowed.sort_unstable_by(|x, y| y.fitness().cmp(&x.fitness()));
        let mut index = 0;
        let mut result: Parents<&T> = Vec::new();
        while index < self.count {
            result.push((borrowed[index], borrowed[index + 1]));
            index += 2;
        }
        Ok(result)
    }
}
