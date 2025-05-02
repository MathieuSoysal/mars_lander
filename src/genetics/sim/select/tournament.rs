use super::*;
use pheno::{Fitness, Phenotype};
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct TournamentSelector {
    count: usize,
    participants: usize,
}

impl TournamentSelector {

    #[deprecated(
        note = "The `TournamentSelector` requires at least 2 participants. This is not enforced
                       by the `new` function. You should use `new_checked` instead.",
        since = "1.7.11"
    )]
    pub fn new(count: usize, participants: usize) -> TournamentSelector {
        TournamentSelector {
            count,
            participants,
        }
    }

    pub fn new_checked(count: usize, participants: usize) -> Result<TournamentSelector, String> {
        if count == 0 || count % 2 != 0 || participants < 2 {
            Err(String::from(
                "count must be larger than zero and a multiple of two; participants must be larger than one",
            ))
        } else {
            Ok(TournamentSelector {
                count,
                participants,
            })
        }
    }
}

impl<T, F> Selector<T, F> for TournamentSelector
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
        if self.participants == 0 || self.participants >= population.len() {
            return Err(format!(
                "Invalid parameter `participants`: {}. Should be larger than \
                 zero and less than the population size.",
                self.participants
            ));
        }

        let mut result: Parents<&T> = Vec::new();
        let mut rng = ::rand::thread_rng();
        for _ in 0..(self.count / 2) {
            // Get tournament participant indices
            let mut participant_indices: Vec<usize> = Vec::with_capacity(self.participants);
            for _ in 0..self.participants {
                let index = rng.gen_range(0..population.len());
                participant_indices.push(index);
            }
            
            // Sort by fitness using clones
            participant_indices.sort_by(|&i, &j| {
                let mut y_clone = population[j].clone();
                let mut x_clone = population[i].clone();
                y_clone.fitness().cmp(&x_clone.fitness())
            });
            
            result.push((&population[participant_indices[0]], &population[participant_indices[1]]));
        }
        Ok(result)
    }
}
