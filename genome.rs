const GENOME_SIZE_BITS: usize = 127;
const ROTATE_SIZE_BITS: usize = 2;
const POWER_SIZE_BITS: usize = 2;
const TURN_SIZE_BITS: usize = POWER_SIZE_BITS + ROTATE_SIZE_BITS;
const TURN_MASK: u8 = ((1 << TURN_SIZE_BITS) - 1) as u8;

use std::arch::x86_64::{
    __m256i, __m512i, _mm_extract_epi32, _mm256_extracti128_si256, _mm256_set_epi32,
};

pub type Genome = __m256i;

// per‑lane 64‑bit masks
const ROTATE_MASK: u8 = (1 << ROTATE_SIZE_BITS) - 1;
const POWER_MASK: u8 = (1 << POWER_SIZE_BITS) - 1;

pub fn genome_init(part1: u128, part2: u128) -> Genome {
    unsafe {
        _mm256_set_epi32(
            (part1 >> 96) as i32,
            (part1 >> 64) as i32,
            (part1 >> 32) as i32,
            (part1 >> 0) as i32,
            (part2 >> 96) as i32,
            (part2 >> 64) as i32,
            (part2 >> 32) as i32,
            (part2 >> 0) as i32,
        )
    }
}

#[inline(always)]
pub fn genome_get_turn(g: Genome, nb_turn: usize) -> u8 {
    // Calculate byte index and bit offset within that byte
    let byte_index = (nb_turn * TURN_SIZE_BITS) / 8;
    let bit_offset = (nb_turn * TURN_SIZE_BITS) % 32;




    // Extract the correct 128-bit lane (high or low)
    let lane = if byte_index >= 16 {
        unsafe { _mm256_extracti128_si256(g, 1) } // High 128 bits
    } else {
        unsafe { _mm256_extracti128_si256(g, 0) } // Low 128 bits
    };

    // Extract the 32-bit chunk containing our byte
    let chunk_index = (byte_index % 16) / 4;
    let chunk = if chunk_index == 0 {
        unsafe { _mm_extract_epi32(lane, 0) }
    } else if chunk_index == 1 {
        unsafe { _mm_extract_epi32(lane, 1) }
    } else if chunk_index == 2 {
        unsafe { _mm_extract_epi32(lane, 2) }
    } else {
        unsafe { _mm_extract_epi32(lane, 3) }
    };

    (chunk >> bit_offset & TURN_MASK as i32) as u8
}

#[inline(always)]
pub fn genome_get_power(g: Genome, nb_turn: usize) -> i8 {
    match genome_get_turn(g, nb_turn) & POWER_MASK as u8 >> ROTATE_SIZE_BITS {
        0 => 0 as i8,
        1 => -1,
        2 => 1,
        _ => 0,
    }
}

#[inline(always)]
pub fn genome_get_rotate(g: Genome, nb_turn: usize) -> i8 {
    let t = genome_get_turn(g, nb_turn);
    match t >> POWER_SIZE_BITS {
        0 => 0 as i8,
        1 => -15,
        2 => 15,
        _ => 0,
    }
}

//tests 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genome_get_turn() {
        let g = genome_init(0x1234_0000_0000_0000, 0x0000_0000_0000_000f);
        assert_eq!(genome_get_turn(g, 0), 0xf);
        assert_eq!(genome_get_turn(g, 1), 0x00);
        assert_eq!(genome_get_turn(g, 2), 0x00);
        assert_eq!(genome_get_turn(g, 3), 0x00);
        assert_eq!(genome_get_turn(g, 4), 0x00);
        assert_eq!(genome_get_turn(g, 5), 0x00);
        assert_eq!(genome_get_turn(g, 6), 0x00);
        assert_eq!(genome_get_turn(g, 7), 0x00);
        assert_eq!(genome_get_turn(g, 8), 0x00);
        assert_eq!(genome_get_turn(g, 9), 0x00);
        assert_eq!(genome_get_turn(g, 10), 0x00);
        assert_eq!(genome_get_turn(g, 11), 0x00);
        assert_eq!(genome_get_turn(g, 12), 0x00);
        assert_eq!(genome_get_turn(g, 13), 0x00);
        assert_eq!(genome_get_turn(g, 14), 0x00);
        assert_eq!(genome_get_turn(g, 15), 0x00);
        assert_eq!(genome_get_turn(g, 16), 0x00);
        assert_eq!(genome_get_turn(g, 17), 0x00);
        assert_eq!(genome_get_turn(g, 18), 0x00);
        assert_eq!(genome_get_turn(g, 19), 0x00);
        assert_eq!(genome_get_turn(g, 20), 0x00);
        assert_eq!(genome_get_turn(g, 21), 0x00);
        assert_eq!(genome_get_turn(g, 22), 0x00);
        assert_eq!(genome_get_turn(g, 23), 0x00);
        assert_eq!(genome_get_turn(g, 24), 0x00);
        assert_eq!(genome_get_turn(g, 25), 0x00);
        assert_eq!(genome_get_turn(g, 26), 0x00);
        assert_eq!(genome_get_turn(g, 27), 0x00);
        assert_eq!(genome_get_turn(g, 28), 0x00);
        assert_eq!(genome_get_turn(g, 29), 0x00);
        assert_eq!(genome_get_turn(g, 30), 0x00);
        assert_eq!(genome_get_turn(g, 31), 0x00);
        assert_eq!(genome_get_turn(g, 32), 0x00);
        assert_eq!(genome_get_turn(g, 33), 0x00);
        assert_eq!(genome_get_turn(g, 34), 0x00);
        assert_eq!(genome_get_turn(g, 35), 0x00);
        assert_eq!(genome_get_turn(g, 36), 0x00);
        assert_eq!(genome_get_turn(g, 37), 0x00);
        assert_eq!(genome_get_turn(g, 38), 0x00);
        assert_eq!(genome_get_turn(g, 39), 0x00);
        assert_eq!(genome_get_turn(g, 40), 0x00);
        assert_eq!(genome_get_turn(g, 41), 0x00);
        assert_eq!(genome_get_turn(g, 42), 0x00);
        assert_eq!(genome_get_turn(g, 43), 0x00);
        assert_eq!(genome_get_turn(g, 44), 0x00);
        assert_eq!(genome_get_turn(g, 45), 0x00);
        assert_eq!(genome_get_turn(g, 46), 0x00);
        assert_eq!(genome_get_turn(g, 47), 0x00);
        assert_eq!(genome_get_turn(g, 48), 0x00);
        assert_eq!(genome_get_turn(g, 49), 0x00);
        assert_eq!(genome_get_turn(g, 50), 0x00);
        assert_eq!(genome_get_turn(g, 51), 0x00);
        assert_eq!(genome_get_turn(g, 52), 0x00);
        assert_eq!(genome_get_turn(g, 53), 0x00);
        assert_eq!(genome_get_turn(g, 54), 0x00);
        assert_eq!(genome_get_turn(g, 55), 0x00);
        assert_eq!(genome_get_turn(g, 56), 0x00);
        assert_eq!(genome_get_turn(g, 57), 0x00);
        assert_eq!(genome_get_turn(g, 58), 0x00);
        assert_eq!(genome_get_turn(g, 59), 0x00);
        assert_eq!(genome_get_turn(g, 60), 0x00);
        assert_eq!(genome_get_turn(g, 61), 0x4);
        assert_eq!(genome_get_turn(g, 62), 0x3);
        assert_eq!(genome_get_turn(g, 63), 0x2);
        assert_eq!(genome_get_turn(g, 64), 0x1);
    }
}
