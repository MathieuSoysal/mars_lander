use super::*;

#[inline(always)]
pub const fn starship_get_x(s: Starship) -> u32 {
    (s & X_MASK) as u32
}

#[inline(always)]
pub const fn starship_get_y(s: Starship) -> u32 {
    ((s >> X_SIZE_BITS) & Y_MASK) as u32
}

#[inline(always)]
pub const fn starship_get_fuel(s: Starship) -> u32 {
    ((s >> (X_SIZE_BITS + Y_SIZE_BITS)) & FUEL_MASK) as u32
}

#[inline(always)]
pub const fn starship_get_rotation(s: Starship) -> i32 {
    ((s >> (X_SIZE_BITS + Y_SIZE_BITS + FUEL_SIZE_BITS)) & ROTATE_MASK) as i32 - MIN_ROTATE
}

#[inline(always)]
pub const fn starship_get_power(s: Starship) -> u32 {
    ((s >> (X_SIZE_BITS + Y_SIZE_BITS + FUEL_SIZE_BITS + ROTATE_SIZE_BITS)) & POWER_MASK) as u32
}

#[inline(always)]
pub const fn starship_get_x_speed(s: Starship) -> i32 {
    ((s >> (X_SIZE_BITS + Y_SIZE_BITS + FUEL_SIZE_BITS + ROTATE_SIZE_BITS + POWER_SIZE_BITS))
        & X_SPEED_MASK) as i32
        - MIN_SPEED
}

#[inline(always)]
pub const  fn starship_get_y_speed(s: Starship) -> i32 {
    ((s >> (X_SIZE_BITS
        + Y_SIZE_BITS
        + FUEL_SIZE_BITS
        + ROTATE_SIZE_BITS
        + POWER_SIZE_BITS
        + X_SPEED_SIZE_BITS))
        & Y_SPEED_MASK) as i32
        - MIN_SPEED
}
