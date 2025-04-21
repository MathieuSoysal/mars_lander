const GEN_POWER_SIZE_BITS: usize = 2;
const GEN_ROTATE_SIZE_BITS: usize = 2;

pub type Gen = [u8; 80];

pub fn gen_init_rand() -> Gen {
    let mut genome = [0; 80];
    for i in 0..80 {
        genome[i] = rand::random::<u8>();
    }
    genome
}

pub fn get_rotate_on_turn(genome: &Gen, nb_turn: usize) -> i8 {
    let turn = genome[nb_turn];
    let rotate = turn & GEN_ROTATE_SIZE_BITS as u8;
    match rotate {
        0 => 0,
        1 => -15,
        2 => 15,
        _ => 0,
    }
}

pub fn get_power_on_turn(genome: &Gen, nb_turn: usize) -> i8 {
    let turn = genome[nb_turn];
    let power = (turn >> GEN_ROTATE_SIZE_BITS) & GEN_POWER_SIZE_BITS as u8;
    match power {
        0 => 0,
        1 => -1,
        2 => 1,
        _ => 0,
    }
}
