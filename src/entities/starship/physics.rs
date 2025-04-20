use super::{Starship, starship_getter::*, starship_setter::*};

const MARS_GRAVITY: f32 = 3.711;
const MAX_SPEED: f32 = 500.;
const MIN_SPEED: f32 = -500.;

// fn apply_movement(starship: Starship) -> Starship {
//     let x_speed = starship_get_x_speed(starship) as f32;
//     let y_speed = starship_get_y_speed(starship) as f32;
//     let rotation = starship_get_rotation(starship) as f32;
//     let power = starship_get_power(starship) as f32;

//     let new_x_speed = (x_speed + (rotation.to_radians().cos() * power))
//         .max(MIN_SPEED)
//         .min(MAX_SPEED);
//     let new_y_speed = (y_speed + (rotation.to_radians().sin() * power) - MARS_GRAVITY)
//         .max(MIN_SPEED)
//         .min(MAX_SPEED);

//     let s = starship_set_y_speed(s, new_y_speed as i32);
//     starship_set_x_speed(s, new_x_speed as i32)
// }
