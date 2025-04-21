use super::Starship;

const MARS_GRAVITY: f32 = 3.711;

impl Starship {
    #[inline(always)]
    pub fn apply_movement(&mut self) {
        let rad = (self.rotation as f32).to_radians();

        let thrust = if (self.power as u16) <= self.fuel {
            self.fuel -= self.power as u16;
            self.power as f32
        } else {
            self.power = 0;
            0.0
        };

        let v0_x = self.x_speed;
        let v0_y = self.y_speed;

        let v1_x = -rad.sin() * thrust;
        let v1_y =  rad.cos() * thrust - MARS_GRAVITY;

        self.add_x_speed(v1_x);
        self.add_y_speed(v1_y);
        self.add_x(v0_x + v1_x * 0.5);
        self.add_y(v0_y + v1_y * 0.5);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_apply_movement() {
        let mut starship = Starship::new(1000, 2700, 10000, 0, 0, 0., 0.);
        for _ in 0..20 {
            starship.apply_movement();
        }
        assert_eq!(starship.get_y(), 1958);
    }

    #[test]
    fn test_apply_movement_with_power1_with_orientation() {
        let mut starship = Starship::new(1000, 2700, 10000, 0, 0, 0., 0.);
        starship.add_power(1);
        starship.add_rotation(15);
        for _ in 0..20 {
            starship.apply_movement();
        }
        assert_eq!(starship.get_y(), 2151);
    }

    #[test]
    fn test_apply_movement_with_power4_with_orientation() {
        let mut starship = Starship::new(2500, 2700, 10000, 0, 0, 0., 0.);
        starship.add_rotation(15);
        for _ in 0..20 {
            starship.add_power(1);
            starship.apply_movement();
        }
        assert_eq!(starship.get_y(), 2621);
        assert_eq!(starship.get_x(), 2322);
    }

    #[test]
    fn test_apply_movement_with_power4_with_orientation_negatif() {
        let mut starship = Starship::new(2500, 2700, 10000, 0, 0, 0., 0.);
        starship.add_rotation(-15);
        for _ in 0..20 {
            starship.add_power(1);
            starship.apply_movement();
        }
        assert_eq!(starship.get_y(), 2621);
        assert_eq!(starship.get_x(), 2678);
    }
}
