use criterion::{Criterion, black_box, criterion_group, criterion_main};
use my_lib::entities::{game::Game, starship::Starship};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("crash detection", |b| {
        b.iter(|| {
            let mut game = Game::new(10);
            game.add_point(0, 1500);
            game.add_point(1000, 2000);
            game.add_point(2000, 500);
            game.add_point(3500, 500);
            game.add_point(5000, 1500);
            game.add_point(6999, 1000);

            for x in 0..7000 {
                for y in 0..3000 {
                    let starship = Starship::new(x, y, 0, 0, 0, 0., 0.);
                    black_box(game.starship_is_crash(&starship));
                }
            }
        })
    });
    c.bench_function("starship movement", |b| {
        b.iter(|| {
            for rotation in -90..=90 {
                for power in 0..=4 {
                    let mut starship = Starship::new(0, 0, 10000, rotation, power, 0., 0.);
                    for _ in 0..50 {
                        black_box(starship.apply_movement());
                    }
                }
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
