use super::*;

const MIN_SPEED: f32 = -500.;
const MAX_SPEED: f32 = 500.;


impl Starship {

    #[inline(always)]
    pub fn add_rotation(&mut self, rotation: i8) {
        let new_rotation = self.rotation + rotation;
        if new_rotation < MIN_ROTATE {
            self.rotation = MIN_ROTATE;
        } else if new_rotation > MAX_ROTATE {
            self.rotation = MAX_ROTATE;
        } else {
            self.rotation = new_rotation;
        }
    }

    #[inline(always)]
    pub fn add_x(&mut self, x: f32) {
        self.x += (x * 100.) as i32;
    }

    #[inline(always)]
    pub fn add_y(&mut self, y: f32) {
        self.y += (y * 100.) as i32;
    }

    #[inline(always)]
    pub fn add_x_speed(&mut self, x_speed: f32) {
        let new_x_speed = self.x_speed + x_speed;
        if new_x_speed < MIN_SPEED {
            self.x_speed = MIN_SPEED;
        } else if new_x_speed > MAX_SPEED {
            self.x_speed = MAX_SPEED;
        } else {
            self.x_speed = new_x_speed;
        }
    }

    #[inline(always)]
    pub fn add_y_speed(&mut self, y_speed: f32) {
        let new_y_speed = self.y_speed + y_speed;
        if new_y_speed < MIN_SPEED {
            self.y_speed = MIN_SPEED;
        } else if new_y_speed > MAX_SPEED {
            self.y_speed = MAX_SPEED;
        } else {
            self.y_speed = new_y_speed;
        }
    }

    #[inline(always)]
    pub fn add_power(&mut self, add_power: i32) {
        if add_power > 0 && self.power < MAX_POWER {
            self.power += 1 as u8;
        } else if add_power < 0 && self.power > 0 {
            self.power -= 1 as u8;
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starship_add_rotation() {
        let mut starship = Starship::new(0, 0, 0, 0, 0, 0., 0.);
        starship.add_rotation(10);
        assert_eq!(starship.get_rotation(), 10);
        starship.add_rotation(-20);
        assert_eq!(starship.get_rotation(), -10);
        starship.add_rotation(MAX_ROTATE +30);
        assert_eq!(starship.get_rotation(), MAX_ROTATE);
    }

    #[test]
    fn test_starship_add_x_speed() {
        let mut starship = Starship::new(0, 0, 0, 0, 0, 0., 0.);
        starship.add_x_speed(50.);
        assert_eq!(starship.get_x_speed(), 50.);
        starship.add_x_speed(-100.);
        assert_eq!(starship.get_x_speed(), -50.);
        starship.add_x_speed(MAX_SPEED * 2.);
        assert_eq!(starship.get_x_speed(), MAX_SPEED);
    }

    #[test]
    fn test_starship_add_y_speed() {
        let mut starship = Starship::new(0, 0, 0, 0, 0, 0., 0.);
        starship.add_y_speed(30.);
        assert_eq!(starship.get_y_speed(), 30.);
        starship.add_y_speed(-50.);
        assert_eq!(starship.get_y_speed(), -20.);
        starship.add_y_speed(MAX_SPEED * 2.);
        assert_eq!(starship.get_y_speed(), MAX_SPEED);
    }

    #[test]
    fn test_starship_add_power() {
        let mut starship = Starship::new(0, 0, 50, 0, 0, 0., 0.);
        starship.add_power(10);
        assert_eq!(starship.get_power(), 1);

        starship.add_power(-5);
        assert_eq!(starship.get_power(), 0);

        starship.add_power(MAX_POWER as i32 + 1);
        assert_eq!(starship.get_power(), 1);

        starship.add_power(0 as i32 - 1);
        assert_eq!(starship.get_power(), 0);
    }
}
