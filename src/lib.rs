use entities::{game::Game, genome::{gen_init_rand, get_power_on_turn, get_rotate_on_turn, DNA}, starship::{self, Starship}};
use genetics::sim::{seq::Simulator, Builder, Simulation};

pub mod entities;
pub mod genetics;

pub fn get_next_move(game: Game, starship: Starship) -> (i8, i8) {
    let mut population: Vec<DNA> = Vec::with_capacity(300);
    let mut rng = ::rand::thread_rng();
    for _ in 0..300 {
        let genome = gen_init_rand();
        let dna = DNA::new(genome, &game, starship);
        population.push(dna);
    }
    #[allow(deprecated)]
    let mut builder = Simulator::builder(&mut population);
    builder
        .with_selector(Box::new(genetics::sim::select::UnstableMaximizeSelector::new(10)))
        .with_max_iters(50);
    let mut s = builder.build();
    s.run();
    let result = s.get().unwrap();
    let rotation = get_rotate_on_turn(result.get_genome(), 0);
    let thrust = get_power_on_turn(result.get_genome(), 0);
    (rotation, thrust)
}
