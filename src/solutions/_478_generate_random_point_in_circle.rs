use rand::prelude::*;

struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
    rng: ThreadRng,
}

impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        let rng = rand::thread_rng();
        Solution {
            radius,
            x_center,
            y_center,
            rng,
        }
    }

    fn rand_point(&mut self) -> Vec<f64> {
        let mut x = self.rng.gen_range(-self.radius, self.radius);
        let mut y = self.rng.gen_range(-self.radius, self.radius);
        while x * x + y * y > self.radius * self.radius {
            x = self.rng.gen_range(-self.radius, self.radius);
            y = self.rng.gen_range(-self.radius, self.radius);
        }
        vec![x + self.x_center, y + self.y_center]
    }
}
