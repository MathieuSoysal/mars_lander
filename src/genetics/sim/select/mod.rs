mod max;
mod max_unstable;
mod stochastic;
mod tournament;

use pheno::{Fitness, Phenotype};
use std::fmt::Debug;

use crate::genetics::pheno;

#[allow(deprecated)]
pub use self::max::MaximizeSelector;
pub use self::max_unstable::UnstableMaximizeSelector;
pub use self::stochastic::StochasticSelector;
pub use self::tournament::TournamentSelector;

pub type Parents<T> = Vec<(T, T)>;

pub trait Selector<T, F>: Debug
where
    T: Phenotype<F>,
    F: Fitness,
{
    fn select<'a>(&self, population: &'a [T]) -> Result<Parents<&'a T>, String>;
}
