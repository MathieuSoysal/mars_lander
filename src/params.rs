use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct SimulationParams {
    pub pop_size: usize,
    pub nb_generations: i32,
    pub crossover_rate: f64,
    pub mutation_rate: f32,
    pub elite_count: usize,
}

// Global static variable to hold the simulation parameters.
// It will be initialized and can be updated multiple times.
static SIMULATION_PARAMS: OnceCell<std::sync::Mutex<SimulationParams>> = OnceCell::new();

/// Retrieves the global simulation parameters.
/// Panics if the parameters have not been initialized yet.
pub fn get_params() -> SimulationParams {
    let lock = SIMULATION_PARAMS
        .get()
        .expect(
            "Simulation parameters have not been initialized. Call `init_simulation_params` first.",
        )
        .lock()
        .expect("Failed to acquire lock on simulation parameters.");
    *lock
}

/// Initializes or updates the global simulation parameters.
/// This function can be called multiple times to update the parameters.
pub(crate) fn set_params(params: SimulationParams) {
    let mutex = SIMULATION_PARAMS.get_or_init(|| std::sync::Mutex::new(params));
    let mut lock = mutex
        .lock()
        .expect("Failed to acquire lock on simulation parameters.");
    *lock = params;
}
