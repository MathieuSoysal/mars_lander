use super::{starship_getter::*, *};

pub const fn starship_add_rotation(s: Starship, rotation: i32) -> Starship {
    let new_rotation = starship_get_rotation(s) + rotation;
    if new_rotation < MIN_ROTATE {
        return starship_set_rotation(s, MIN_ROTATE);
    }
    if new_rotation > MAX_ROTATE {
        return starship_set_rotation(s, MAX_ROTATE);
    }
    starship_set_rotation(s, new_rotation)
}

pub const fn starship_add_x_speed(s: Starship, x_speed: i32) -> Starship {
    let new_x_speed = starship_get_x_speed(s) + x_speed;
    if new_x_speed < MIN_SPEED {
        return starship_set_x_speed(s, MIN_SPEED);
    }
    if new_x_speed > MAX_SPEED {
        return starship_set_x_speed(s, MAX_SPEED);
    }
    starship_set_x_speed(s, new_x_speed)
}

pub const fn starship_add_y_speed(s: Starship, y_speed: i32) -> Starship {
    let new_y_speed = starship_get_y_speed(s) + y_speed;
    if new_y_speed < MIN_SPEED {
        return starship_set_y_speed(s, MIN_SPEED);
    }
    if new_y_speed > MAX_SPEED {
        return starship_set_y_speed(s, MAX_SPEED);
    }
    starship_set_y_speed(s, new_y_speed)
}

// 1 power = 1 fuel, si pas assez crash
pub const fn starship_add_power(s: Starship, power: i32) -> Starship {
    if power == 0 {
        return s;
    }
    let current_power = starship_get_power(s);
    if power > 0 && current_power < MAX_POWER {
        starship_set_power(s, current_power + power as u32)
    } else if power < 0 {
        starship_set_power(s, current_power - power.abs() as u32)
    } else {
        s
    }
}
