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
    ((s >> (X_SIZE_BITS + Y_SIZE_BITS + FUEL_SIZE_BITS)) & ROTATE_MASK) as i32 - MAX_ROTATE
}

#[inline(always)]
pub const fn starship_get_power(s: Starship) -> u32 {
    ((s >> (X_SIZE_BITS + Y_SIZE_BITS + FUEL_SIZE_BITS + ROTATE_SIZE_BITS)) & POWER_MASK) as u32
}

#[inline(always)]
pub const fn starship_get_x_speed(s: Starship) -> i32 {
    ((s >> (X_SIZE_BITS + Y_SIZE_BITS + FUEL_SIZE_BITS + ROTATE_SIZE_BITS + POWER_SIZE_BITS))
        & X_SPEED_MASK) as i32
        - MAX_SPEED
}

#[inline(always)]
pub const fn starship_get_y_speed(s: Starship) -> i32 {
    ((s >> (X_SIZE_BITS
        + Y_SIZE_BITS
        + FUEL_SIZE_BITS
        + ROTATE_SIZE_BITS
        + POWER_SIZE_BITS
        + X_SPEED_SIZE_BITS))
        & Y_SPEED_MASK) as i32
        - MAX_SPEED
}

#[cfg(test)]
mod tests {
    use crate::entities::starship;

    use super::*;

    const TEST_STARSHIP: Starship = 0b0110_0011_0100_1111_1010_1100_0011_1010;

    #[test]
    fn test_starship_get_x() {
        let s = starship_set_x(0, 50);

        assert_eq!(starship_get_x(s), 50);
    }

    #[test]
    fn test_starship_get_y() {
        let s = starship_set_y(0, 50);

        assert_eq!(starship_get_y(s), 50);
    }

    #[test]
    fn test_starship_get_fuel() {
        let s = starship_set_fuel(0, 50);

        assert_eq!(starship_get_fuel(s), 50);
    }

    #[test]
    fn test_starship_get_rotation() {
        let s = starship_set_rotation(0, 50);
        assert_eq!(starship_get_rotation(s), 50);
        let s = starship_set_rotation(0, -50);
        assert_eq!(starship_get_rotation(s), -50);
        let s = starship_set_rotation(0, 90);
        assert_eq!(starship_get_rotation(s), 90);
        let s = starship_set_rotation(0, -90);
        assert_eq!(starship_get_rotation(s), -90);
    }

    #[test]
    fn test_starship_get_power() {
        let s = starship_set_power(0, 4);
        assert_eq!(starship_get_power(s), 4);
    }

    #[test]
    fn test_starship_get_x_speed() {
        let s = starship_set_x_speed(0, 50);
        assert_eq!(starship_get_x_speed(s), 50);
        let s = starship_set_x_speed(0, -50);
        assert_eq!(starship_get_x_speed(s), -50);
    }

    #[test]
    fn test_starship_get_y_speed() {
        let s = starship_set_y_speed(0, 50);
        assert_eq!(starship_get_y_speed(s), 50);
        let s = starship_set_y_speed(0, -50);
        assert_eq!(starship_get_y_speed(s), -50);
    }
}
