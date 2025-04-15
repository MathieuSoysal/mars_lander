pub mod starship_adder;
use starship_setter::*;

pub mod starship_getter;
pub mod starship_setter;
pub type Starship = u128;

const X_SIZE_BITS: u32 = 13;
const Y_SIZE_BITS: u32 = 12;
const FUEL_SIZE_BITS: u32 = 11;
const ROTATE_SIZE_BITS: u32 = 8;
const POWER_SIZE_BITS: u32 = 3;
const Y_SPEED_SIZE_BITS: u32 = 10;
const X_SPEED_SIZE_BITS: u32 = 10;

const X_MASK: Starship = (1 << X_SIZE_BITS) - 1;
const Y_MASK: Starship = (1 << Y_SIZE_BITS) - 1;
const FUEL_MASK: Starship = (1 << FUEL_SIZE_BITS) - 1;
const ROTATE_MASK: Starship = (1 << ROTATE_SIZE_BITS) - 1;
const POWER_MASK: Starship = (1 << POWER_SIZE_BITS) - 1;
const Y_SPEED_MASK: Starship = (1 << Y_SPEED_SIZE_BITS) - 1;
const X_SPEED_MASK: Starship = (1 << X_SPEED_SIZE_BITS) - 1;

const MIN_ROTATE: i32 = -90;
const MAX_ROTATE: i32 = 90;
const MIN_SPEED: i32 = -499;
const MAX_SPEED: i32 = 499;
const MAX_POWER: u32 = 4;
const MAX_X: u32 = 6999;
const MAX_Y: u32 = 2999;

pub fn starship_init(
    x: u32,
    y: u32,
    fuel: u32,
    rotation: i32,
    power: u32,
    x_speed: i32,
    y_speed: i32,
) -> Starship {
    starship_set_x(
        starship_set_y(
            starship_set_fuel(
                starship_set_rotation(
                    starship_set_power(
                        starship_set_x_speed(starship_set_y_speed(0, y_speed), x_speed),
                        power,
                    ),
                    rotation,
                ),
                fuel,
            ),
            y,
        ),
        x,
    )
}
