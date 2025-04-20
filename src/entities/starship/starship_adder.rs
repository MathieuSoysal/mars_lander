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

pub const fn starship_add_x_speed(s: Starship, x_speed: f32) -> Starship {
    let new_x_speed = starship_get_x_speed(s) + x_speed;
    if new_x_speed < MIN_SPEED {
        return starship_set_x_speed(s, MIN_SPEED);
    }
    if new_x_speed > MAX_SPEED {
        return starship_set_x_speed(s, MAX_SPEED);
    }
    starship_set_x_speed(s, new_x_speed)
}

pub const fn starship_add_y_speed(s: Starship, y_speed: f32) -> Starship {
    let new_y_speed = starship_get_y_speed(s) + y_speed;
    if new_y_speed < MIN_SPEED {
        return starship_set_y_speed(s, MIN_SPEED);
    }
    if new_y_speed > MAX_SPEED {
        return starship_set_y_speed(s, MAX_SPEED);
    }
    starship_set_y_speed(s, new_y_speed)
}

const fn starship_consum_fuel(s: Starship) -> Starship {
    let power = starship_get_power(s) as i32;
    if power == 0 {
        return s;
    }
    let new_fuel = starship_get_fuel(s) as i32 - power;
    if new_fuel < 0 {
        starship_set_fuel(s, 0)
    } else {
        starship_set_fuel(s, new_fuel as u32)
    }
}

pub fn starship_add_power(s: Starship, add_power: i32) -> Starship {
    if add_power == 0 {
        return starship_consum_fuel(s);
    }
    let current_power = starship_get_power(s);
    let s = if add_power > 0 && current_power < MAX_POWER {
        starship_set_power(s, (current_power + add_power as u32).min(MAX_POWER))
    } else if add_power < 0 {
        starship_set_power(s, current_power.saturating_sub(add_power.abs() as u32))
    } else {
        s
    };
    starship_consum_fuel(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starship_add_rotation() {
        let starship = 0; // Assuming a constructor exists
        let starship = starship_set_rotation(starship, 0);
        let rotated_starship = starship_add_rotation(starship, 10);
        assert_eq!(starship_get_rotation(rotated_starship), 10);

        let rotated_starship = starship_add_rotation(rotated_starship, -20);
        assert_eq!(starship_get_rotation(rotated_starship), -10);

        let rotated_starship = starship_add_rotation(rotated_starship, MAX_ROTATE * 2);
        assert_eq!(starship_get_rotation(rotated_starship), MAX_ROTATE);
    }

    #[test]
    fn test_starship_add_x_speed() {
        let starship = starship_set_x_speed(0, 0.);
        let faster_starship = starship_add_x_speed(starship, 50.);
        assert_eq!(starship_get_x_speed(faster_starship), 50.);

        let faster_starship = starship_add_x_speed(faster_starship, -100.);
        assert_eq!(starship_get_x_speed(faster_starship), -50.);

        let faster_starship = starship_add_x_speed(faster_starship, MAX_SPEED * 2.);
        assert_eq!(starship_get_x_speed(faster_starship), MAX_SPEED);
    }

    #[test]
    fn test_starship_add_y_speed() {
        let starship = starship_set_y_speed(0, 0.);
        let faster_starship = starship_add_y_speed(starship, 30.);
        assert_eq!(starship_get_y_speed(faster_starship), 30.);

        let faster_starship = starship_add_y_speed(faster_starship, -50.);
        assert_eq!(starship_get_y_speed(faster_starship), -20.);

        let faster_starship = starship_add_y_speed(faster_starship, MAX_SPEED * 2.);
        assert_eq!(starship_get_y_speed(faster_starship), MAX_SPEED);
    }

    #[test]
    fn test_starship_add_power() {
        let starship = starship_set_power(0, 0);
        let powered_starship = starship_add_power(starship, 10);
        assert_eq!(starship_get_power(powered_starship), 4);

        let powered_starship = starship_add_power(powered_starship, -5);
        assert_eq!(starship_get_power(powered_starship), 0);

        let powered_starship = starship_add_power(powered_starship, MAX_POWER as i32 + 1);
        assert_eq!(starship_get_power(powered_starship), 4);

        let powered_starship = starship_add_power(powered_starship, 0 as i32 - 1);
        assert_eq!(starship_get_power(powered_starship), 3);
    }
}
