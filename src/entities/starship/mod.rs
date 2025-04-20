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

const X_SIZE_BITS: u32 = 13;
const Y_SIZE_BITS: u32 = 12;
const FUEL_SIZE_BITS: u32 = 11;
const ROTATE_SIZE_BITS: u32 = 8;
const POWER_SIZE_BITS: u32 = 3;
const Y_SPEED_SIZE_BITS: u32 = 19;
const X_SPEED_SIZE_BITS: u32 = 19;
const SPEED_PRECISION: f32 = 100.;

const MIN_ROTATE: i8 = -90;
const MAX_ROTATE: i8 = 90;
const MIN_SPEED: f32 = -500.;
const MAX_SPEED: f32 = 500.;

const MAX_POWER: u8 = 4;
const MAX_X: u32 = 6999;
const MAX_Y: u32 = 2999;

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
        self.x.round() as u32
    }

    pub fn get_y(&self) -> u32 {
        self.y.round() as u32
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



