use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct SimulationParams {
    pub pop_size: usize,
    pub nb_generations: i32,
    pub crossover_rate: f64,
    pub mutation_rate: f64,
    pub elite_rate: f64,
}
