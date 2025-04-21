mod physics;
pub mod starship_adder;

pub struct Starship {
    pub x: f32,
    pub y: f32,
    pub fuel: u16,
    pub rotation: i8,
    pub power: u8,
    pub x_speed: f32,
    pub y_speed: f32,
}

const MIN_ROTATE: i8 = -90;
const MAX_ROTATE: i8 = 90;

const MAX_POWER: u8 = 4;


impl Starship {

    pub fn new(x: f32, y: f32, fuel: u16, rotation: i8, power: u8, x_speed: f32, y_speed: f32) -> Self {
        Starship {
            x,
            y,
            fuel,
            rotation,
            power,
            x_speed,
            y_speed,
        }
    }

    pub fn get_x(&self) -> u32 {
        (self.x + 0.5) as u32
    }

    pub fn get_y(&self) -> u32 {
        (self.y + 0.5) as u32
    }

    pub fn get_fuel(&self) -> u16 {
        self.fuel
    }

    pub fn get_rotation(&self) -> i8 {
        self.rotation
    }

    pub fn get_power(&self) -> u8 {
        self.power
    }

    pub fn get_x_speed(&self) -> f32 {
        self.x_speed
    }

    pub fn get_y_speed(&self) -> f32 {
        self.y_speed
    }
}



