use super::*;

pub const fn starship_set_x(s: Starship, x: u32) -> Starship {
    (s & !X_MASK) | (x as Starship & X_MASK)
}

pub const fn starship_set_y(s: Starship, y: u32) -> Starship {
    (s & !(Y_MASK << X_SIZE_BITS)) | ((y as Starship & Y_MASK) << X_SIZE_BITS)
}

pub const fn starship_set_fuel(s: Starship, fuel: u32) -> Starship {
    (s & !(FUEL_MASK << (X_SIZE_BITS + Y_SIZE_BITS)))
        | ((fuel as Starship & FUEL_MASK) << (X_SIZE_BITS + Y_SIZE_BITS))
}

pub const fn starship_set_rotation(s: Starship, rotation: i32) -> Starship {
    (s & !(ROTATE_MASK << (X_SIZE_BITS + Y_SIZE_BITS + FUEL_SIZE_BITS)))
        | (((rotation + MAX_ROTATE) as Starship & ROTATE_MASK)
            << (X_SIZE_BITS + Y_SIZE_BITS + FUEL_SIZE_BITS))
}

pub const fn starship_set_power(s: Starship, power: u32) -> Starship {
    (s & !(POWER_MASK << (X_SIZE_BITS + Y_SIZE_BITS + FUEL_SIZE_BITS + ROTATE_SIZE_BITS)))
        | (((if power > MAX_POWER { MAX_POWER } else { power }) as Starship)
            << (X_SIZE_BITS + Y_SIZE_BITS + FUEL_SIZE_BITS + ROTATE_SIZE_BITS))
}

pub const fn starship_set_x_speed(s: Starship, x_speed: i32) -> Starship {
    (s & !(X_SPEED_MASK
        << (X_SIZE_BITS + Y_SIZE_BITS + FUEL_SIZE_BITS + ROTATE_SIZE_BITS + POWER_SIZE_BITS)))
        | (((x_speed + MAX_SPEED) as Starship & X_SPEED_MASK)
            << (X_SIZE_BITS + Y_SIZE_BITS + FUEL_SIZE_BITS + ROTATE_SIZE_BITS + POWER_SIZE_BITS))
}

pub const fn starship_set_y_speed(s: Starship, y_speed: i32) -> Starship {
    (s & !(Y_SPEED_MASK
        << (X_SIZE_BITS
            + Y_SIZE_BITS
            + FUEL_SIZE_BITS
            + ROTATE_SIZE_BITS
            + POWER_SIZE_BITS
            + X_SPEED_SIZE_BITS)))
        | (((y_speed + MAX_SPEED) as Starship & Y_SPEED_MASK)
            << (X_SIZE_BITS
                + Y_SIZE_BITS
                + FUEL_SIZE_BITS
                + ROTATE_SIZE_BITS
                + POWER_SIZE_BITS
                + X_SPEED_SIZE_BITS))
}
