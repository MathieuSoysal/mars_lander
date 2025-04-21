pub trait Fitness: Ord + Eq {
    fn zero() -> Self;
    fn abs_diff(&self, other: &Self) -> Self;
}
pub trait Phenotype<F>: Clone
where
    F: Fitness,
{
    fn fitness(&self) -> F;
    fn crossover(&self, other: &Self) -> Self;
    fn mutate(&self) -> Self;
}
