use std::collections::HashMap;

#[derive(Default)]
struct UndergroundSystem {
    time: HashMap<String, HashMap<String, (i32, i32)>>,
    customer: HashMap<i32, (String, i32)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        UndergroundSystem::default()
    }

    fn check_in(&mut self, id: i32, start_station: String, start_t: i32) {
        self.customer.insert(id, (start_station, start_t));
    }

    fn check_out(&mut self, id: i32, end_station: String, end_t: i32) {
        let (start_station, start_t) = self.customer.remove(&id).expect("in");
        let (sum, count) = self
            .time
            .entry(start_station)
            .or_default()
            .entry(end_station)
            .or_default();
        *sum += end_t - start_t;
        *count += 1;
    }

    fn get_average_time(&mut self, start_station: String, end_station: String) -> f64 {
        let (sum, count) = self
            .time
            .entry(start_station)
            .or_default()
            .entry(end_station)
            .or_default();
        *sum as f64 / *count as f64
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let mut obj = UndergroundSystem::new();
    obj.check_in(45, "Leyton".to_string(), 3);
    obj.check_in(32, "Paradise".to_string(), 8);
    obj.check_in(27, "Leyton".to_string(), 10);
    obj.check_out(45, "Waterloo".to_string(), 15);
    obj.check_out(27, "Waterloo".to_string(), 20);
    obj.check_out(32, "Cambridge".to_string(), 22);
    let t = obj.get_average_time("Paradise".to_string(), "Cambridge".to_string());
    assert_approx_eq!(t, 14.0);
    let t = obj.get_average_time("Leyton".to_string(), "Waterloo".to_string());
    assert_approx_eq!(t, 11.0);
    obj.check_in(10, "Leyton".to_string(), 24);
    let t = obj.get_average_time("Leyton".to_string(), "Waterloo".to_string());
    assert_approx_eq!(t, 11.0);
    obj.check_out(10, "Waterloo".to_string(), 38);
    let t = obj.get_average_time("Leyton".to_string(), "Waterloo".to_string());
    assert_approx_eq!(t, 12.0);
}
