mod physics;
pub mod starship_adder;

pub struct Starship {
    pub x: i32,
    pub y: i32,
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

    pub fn new(x: i32, y: i32, fuel: u16, rotation: i8, power: u8, x_speed: f32, y_speed: f32) -> Self {
        Starship {
            x : (x * 100),
            y : (y * 100),
            fuel,
            rotation,
            power,
            x_speed,
            y_speed,
        }
    }

    #[inline(always)]
    pub fn get_x(&self) -> i32 {
        (self.x + 50) / 100 
    }

    #[inline(always)]
    pub fn get_y(&self) -> i32 {
        (self.y + 50) / 100
    }

    #[inline(always)]
    pub fn get_fuel(&self) -> u16 {
        self.fuel
    }

    #[inline(always)]
    pub fn get_rotation(&self) -> i8 {
        self.rotation
    }

    #[inline(always)]
    pub fn get_power(&self) -> u8 {
        self.power
    }

    #[inline(always)]
    pub fn get_x_speed(&self) -> f32 {
        self.x_speed
    }

    #[inline(always)]
    pub fn get_y_speed(&self) -> f32 {
        self.y_speed
    }
}



