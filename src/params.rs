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
// It will be initialized once via a function called from JavaScript.
static SIMULATION_PARAMS: OnceCell<SimulationParams> = OnceCell::new();

/// Retrieves the global simulation parameters.
/// Panics if the parameters have not been initialized yet.
pub fn get_params() -> SimulationParams {
    *SIMULATION_PARAMS.get().expect(
        "Simulation parameters have not been initialized. Call `init_simulation_params` first.",
    )
}

/// Initializes the global simulation parameters.
/// This function should only be called once, typically from JavaScript.
/// Returns Ok(()) if initialization was successful, Err(params) if already initialized.
pub(crate) fn init_params(params: SimulationParams) -> Result<(), SimulationParams> {
    SIMULATION_PARAMS.set(params)
}
