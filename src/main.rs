extern crate my_lib;

fn main() {
    let mut game = my_lib::entities::game::Game::new(10);
    game.add_point(0, 1500);
    game.add_point(1000, 2000);
    game.add_point(2000, 500);
    game.add_point(3500, 500);
    game.add_point(5000, 1500);
    game.add_point(6999, 1000);

    let starship = my_lib::entities::starship::Starship::new(2500, 2700, 5500, 0, 0, 0., 0.);
    let (rotation, thrust) = my_lib::get_next_move(game, starship);
    println!("{} {}", rotation, thrust);
}
