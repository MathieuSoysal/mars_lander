use super::Starship;

const MARS_GRAVITY: f32 = 3.711;

impl Starship {
    pub fn apply_movement(&mut self) {
        let v0 = self.y_speed;

        let v1 = -MARS_GRAVITY;

        self.y_speed += v1;

        let calculated_y = v0 + (v1 / 2.);

        self.y += calculated_y;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_apply_movement() {
        let mut starship = Starship::new(1000., 2700., 10000, 0, 0, 0., 0.);
        for i in 0..20 {
            starship.apply_movement();
            println!(
                "{i} x: {}, y: {}",
                starship.get_x(),
                starship.get_y()
            );
        }
        assert_eq!(starship.get_y(), 1958);
    }
}
