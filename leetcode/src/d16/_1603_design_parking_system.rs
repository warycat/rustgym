struct ParkingSystem {
    sizes: Vec<i32>,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        let sizes = vec![big, medium, small];
        ParkingSystem { sizes }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let index = (car_type - 1) as usize;
        if self.sizes[index] > 0 {
            self.sizes[index] -= 1;
            true
        } else {
            false
        }
    }
}

#[test]
fn test() {
    let mut obj = ParkingSystem::new(1, 1, 0);
    assert_eq!(obj.add_car(1), true);
    assert_eq!(obj.add_car(2), true);
    assert_eq!(obj.add_car(3), false);
    assert_eq!(obj.add_car(1), false);
}
