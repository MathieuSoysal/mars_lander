const GENOME_SIZE_BITS: usize = 127;
const ROTATE_SIZE_BITS: usize = 2;
const POWER_SIZE_BITS: usize = 2;
const TURN_SIZE_BITS: usize = POWER_SIZE_BITS + ROTATE_SIZE_BITS;

pub type Genome = u128;

const ROTATE_MASK: Genome = (1 << ROTATE_SIZE_BITS) - 1;
const POWER_MASK: Genome = (1 << POWER_SIZE_BITS) - 1;
const TURN_MASK: Genome = (1 << TURN_SIZE_BITS) - 1;
const GENOME_MASK: Genome = (1 << GENOME_SIZE_BITS) - 1;

#[inline(always)]
const fn get_turn(g: Genome, nb_turn: usize) -> u8 {
    ((g >> (nb_turn * TURN_SIZE_BITS)) & TURN_MASK) as u8
}

#[inline(always)]
pub const fn get_power(g: Genome, nb_turn: usize) -> u8 {
    get_turn(g, nb_turn) & POWER_MASK as u8
}

#[inline(always)]
pub const fn get_rotate(g: Genome, nb_turn: usize) -> u8 {
    (get_turn(g, nb_turn) >> POWER_SIZE_BITS) as u8
}
