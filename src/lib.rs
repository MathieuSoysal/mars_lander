pub mod my_genetics;

use entities::{
    game::Game,
    genome::{gen_init_full, gen_init_rand, gen_init_semi_full, get_power_on_turn, get_rotate_on_turn, DNA},
    starship::{self, Starship},
};
use genetics::sim::{Builder, Simulation, seq::Simulator};

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
        .with_selector(Box::new(
            genetics::sim::select::UnstableMaximizeSelector::new(10),
        ))
        .with_max_iters(50);
    let mut s = builder.build();
    s.run();
    let result = s.get().unwrap();
    let rotation = get_rotate_on_turn(result.get_genome(), 0);
    let thrust = get_power_on_turn(result.get_genome(), 0);
    (rotation, thrust)
}

// pub fn tes() {
//     let mut game = Game::new(10);
//     let mut starship = Starship::new(6500, 1900, 10000, 0, 0, 0., 0.);
//     game.add_point(0, 1500);
//     game.add_point(1000, 2000);
//     game.add_point(2000, 500);
//     game.add_point(3500, 500);
//     game.add_point(5000, 1500);
//     game.add_point(6999, 1000);
//     let start_time = std::time::Instant::now();

//     let mut population: [DNA; 100] = array::from_fn(|_| {
//         let genome = gen_init_rand();
//         DNA::new(genome, &game, starship.copy())
//     });
//     let mut new_population: [DNA; 100] = array::from_fn(|_| {
//         let genome = gen_init_rand();
//         DNA::new(genome, &game, starship.copy())
//     });
//     population[0] = DNA::new(
//         gen_init_full(),
//         &game,
//         starship.copy(),
//     );
//     population[1] = DNA::new(
//         gen_init_semi_full(),
//         &game,
//         starship.copy(),
//     );
//     for i in 0..300 {
//         elitiste_new_population(
//             &mut population,
//             &mut new_population,
//             10,
//             0.6,
//         );
//         // population_to_svg(&population, i);
//         population = new_population;
//     }
//     let best = population[0].clone();
//     let rot = get_rotate_on_turn(best.get_genome(), 0);
//     starship.add_rotation(rot);
//     let thrust = get_power_on_turn(best.get_genome(), 0);
//     starship.add_power(thrust as i32);
//     println!("{} {}", starship.get_rotation(), starship.get_power());
// }
