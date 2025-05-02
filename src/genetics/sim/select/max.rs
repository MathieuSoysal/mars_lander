#![allow(deprecated)]
use crate::genetics::pheno;

use super::*;
use pheno::{Fitness, Phenotype};

#[derive(Clone, Copy, Debug)]
#[deprecated(
    note = "The `MaximizeSelector` has bad performance due to sorting. For better performance with potentially different results, \
                   use the `UnstableMaximizeSelector`.",
    since = "1.7.7"
)]
pub struct MaximizeSelector {
    count: usize,
}

impl MaximizeSelector {
    pub fn new(count: usize) -> MaximizeSelector {
        MaximizeSelector { count }
    }
}

impl<T, F> Selector<T, F> for MaximizeSelector
where
    T: Phenotype<F> + Clone,
    F: Fitness,
{
    fn select<'a>(&self, population: &'a [T]) -> Result<Parents<&'a T>, String> {
        if self.count == 0 || self.count % 2 != 0 || self.count * 2 >= population.len() {
            return Err(format!(
                "Invalid parameter `count`: {}. Should be larger than zero, a \
                 multiple of two and less than half the population size.",
                self.count
            ));
        }

        // Sort indices instead of borrowing directly
        let mut indices: Vec<usize> = (0..population.len()).collect();
        indices.sort_by(|&i, &j| {
            let mut y_clone = population[j].clone();
            let mut x_clone = population[i].clone();
            y_clone.fitness().cmp(&x_clone.fitness())
        });
        
        let mut index = 0;
        let mut result: Parents<&T> = Vec::new();
        while index < self.count {
            result.push((&population[indices[index]], &population[indices[index + 1]]));
            index += 2;
        }
        Ok(result)
    }
}

