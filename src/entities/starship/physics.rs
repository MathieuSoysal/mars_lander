use super::Starship;

const MARS_GRAVITY: f32 = 3.711;

impl Starship {
    pub fn apply_movement(&mut self) {
        let v0_y = self.y_speed;
        let v0_x = self.x_speed;
        let rotation = self.rotation as f32;
        let power = self.power as f32;

        let v1_x = -1. * rotation.to_radians().sin() * power;
        let v1_y = rotation.to_radians().cos() * power - MARS_GRAVITY;

        self.y_speed += v1_y;
        if self.y_speed < -500. {
            self.y_speed = -500.;
        } else if self.y_speed > 500. {
            self.y_speed = 500.;
        }
        self.x_speed += v1_x;
        if self.x_speed < -500. {
            self.x_speed = -500.;
        } else if self.x_speed > 500. {
            self.x_speed = 500.;
        }

        self.y += v0_y + (v1_y / 2.);
        self.x += v0_x + (v1_x / 2.);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_apply_movement() {
        let mut starship = Starship::new(1000., 2700., 10000, 0, 0, 0., 0.);
        for _ in 0..20 {
            starship.apply_movement();
        }
        assert_eq!(starship.get_y(), 1958);
    }

    #[test]
    fn test_apply_movement_with_power1_with_orientation() {
        let mut starship = Starship::new(1000., 2700., 10000, 0, 0, 0., 0.);
        starship.add_power(1);
        starship.add_rotation(15);
        for _ in 0..20 {
            starship.apply_movement();
        }
        assert_eq!(starship.get_y(), 2151);
    }

    #[test]
    fn test_apply_movement_with_power4_with_orientation() {
        let mut starship = Starship::new(2500., 2700., 10000, 0, 0, 0., 0.);
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
        let mut starship = Starship::new(2500., 2700., 10000, 0, 0, 0., 0.);
        starship.add_rotation(-15);
        for _ in 0..20 {
            starship.add_power(1);
            starship.apply_movement();
        }
        assert_eq!(starship.get_y(), 2621);
        assert_eq!(starship.get_x(), 2678);
    }
}
