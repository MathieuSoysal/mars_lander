use pheno::{Fitness, Phenotype};

use super::pheno;

mod earlystopper;
mod iterlimit;
pub mod select;
pub mod seq;
pub mod types;

pub trait Builder<T: ?Sized> {
    fn build(self) -> T
    where
        T: Sized;
}

pub type NanoSecond = i64;
pub type SimResult<'a, T> = Result<&'a T, &'a str>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StepResult {
    Success,
    Failure,
    Done,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RunResult {
    Failure,
    Done,
}

pub trait Simulation<'a, T, F>
where
    T: Phenotype<F>,
    F: Fitness,
{
    type B: Builder<Self>;

    fn builder(population: &'a mut Vec<T>) -> Self::B
    where
        Self: Sized;
    fn run(&mut self) -> RunResult;
    #[deprecated(
        note = "To encourage checking the `StepResult` while maintaining backwards \
                compatibility, this function has been deprecated in favour of `checked_step`.",
        since = "1.7.0"
    )]
    fn step(&mut self) -> StepResult;
    fn checked_step(&mut self) -> StepResult;
    fn get(&'a self) -> SimResult<'a, T>;
    fn time(&self) -> Option<NanoSecond>;
    fn iterations(&self) -> u64;
    fn population(&self) -> Vec<T>;
}
