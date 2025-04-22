
pub trait Selector<T> {
    fn select(&self, population: &mut [T]) -> [T];
}
