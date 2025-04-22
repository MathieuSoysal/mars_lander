use std::convert::TryInto;
use std::{array, io};

extern crate my_lib;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // the number of points used to draw the surface of Mars.
    let mut game = my_lib::entities::game::Game::new(n as usize);
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let land_x = parse_input!(inputs[0], i32); // X coordinate of a surface point. (0 to 6999)
        let land_y = parse_input!(inputs[1], i32); // Y coordinate of a surface point. By linking all the points together in a sequential fashion, you form the surface of Mars.
        game.add_point(land_x as usize, land_y as usize);
    }
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let hs = parse_input!(inputs[2], i32); // the horizontal speed (in m/s), can be negative.
        let vs = parse_input!(inputs[3], i32); // the vertical speed (in m/s), can be negative.
        let f = parse_input!(inputs[4], i32); // the quantity of remaining fuel in liters.
        let r = parse_input!(inputs[5], i32); // the rotation angle in degrees (-90 to 90).
        let p = parse_input!(inputs[6], i32); // the thrust power (0 to 4).

        let start_time = std::time::Instant::now();

        let mut starship = my_lib::entities::starship::Starship::new(
            x,
            y,
            f.try_into().unwrap(),
            r.try_into().unwrap(),
            p.try_into().unwrap(),
            vs as f32,
            hs as f32,
        );

        let mut population: [my_lib::entities::genome::DNA; 100] = array::from_fn(|_| {
            let genome = my_lib::entities::genome::gen_init_rand();
            my_lib::entities::genome::DNA::new(genome, &game, starship.copy())
        });
        let mut new_population: [my_lib::entities::genome::DNA; 100] = array::from_fn(|_| {
            let genome = my_lib::entities::genome::gen_init_rand();
            my_lib::entities::genome::DNA::new(genome, &game, starship.copy())
        });
        population[0] = my_lib::entities::genome::DNA::new(
            my_lib::entities::genome::gen_init_full(),
            &game,
            starship.copy(),
        );
        population[1] = my_lib::entities::genome::DNA::new(
            my_lib::entities::genome::gen_init_semi_full(),
            &game,
            starship.copy(),
        );
        while start_time.elapsed() < std::time::Duration::from_millis(90) {
            my_lib::my_genetics::roulette::roulette_new_population(
                &population,
                &mut new_population,
                0.2,
            );
            population = new_population;
        }
        let rot = my_lib::entities::genome::get_rotate_on_turn(population[0].get_genome(), 0);
        starship.add_rotation(rot);
        let thrust = my_lib::entities::genome::get_power_on_turn(population[0].get_genome(), 0);
        starship.add_power(thrust as i32);
        println!("{} {}", starship.get_rotation(), starship.get_power());
    }
}
