use std::sync::Mutex;

struct TrafficLight {
    green: Mutex<i32>,
}

impl TrafficLight {
    fn new() -> Self {
        TrafficLight {
            green: Mutex::new(1),
        }
    }

    fn road_name(road_id: i32) -> String {
        if road_id == 1 {
            return "A".to_string();
        }
        if road_id == 2 {
            return "B".to_string();
        }
        panic!()
    }

    fn road_id(direction: i32) -> i32 {
        if direction == 1 || direction == 2 {
            return 1;
        }
        if direction == 3 || direction == 4 {
            return 2;
        }
        panic!()
    }

    fn car_arrived(
        &self,
        _car_id: i32,
        road_id: i32,
        _direction: i32,
        turn_green: impl FnOnce(),
        cross_car: impl FnOnce(),
    ) {
        if let Ok(mut g) = self.green.lock() {
            if *g != road_id {
                *g = road_id;
                turn_green();
            }
            cross_car();
        }
    }
}

#[test]
fn test() {
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    let n = 5;
    let cars = vec![1, 3, 5, 2, 4];
    let directions = vec![2, 1, 2, 4, 3];
    let arrival_times = vec![10, 20, 30, 40, 50];
    let ans = vec_string![
        "Car 1 Has Passed Road A In Direction 2",
        "Car 3 Has Passed Road A In Direction 1",
        "Car 5 Has Passed Road A In Direction 2",
        "Traffic Light On Road B Is Green",
        "Car 2 Has Passed Road B In Direction 4",
        "Car 4 Has Passed Road B In Direction 3"
    ];

    let (tx, rx) = channel();
    let mut threads = vec![];
    let traffic_light = Arc::new(TrafficLight::new());
    for i in 0..n {
        let tx = tx.clone();
        let car_id = cars[i];
        let direction = directions[i];
        let road_id = TrafficLight::road_id(direction);
        let arrival_time = arrival_times[i];
        let traffic_light = traffic_light.clone();
        threads.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(arrival_time));
            traffic_light.car_arrived(
                car_id,
                road_id,
                direction,
                || {
                    tx.send(format!(
                        "Traffic Light On Road {} Is Green",
                        TrafficLight::road_name(road_id)
                    ))
                    .unwrap();
                },
                || {
                    tx.send(format!(
                        "Car {} Has Passed Road {} In Direction {}",
                        car_id,
                        TrafficLight::road_name(road_id),
                        direction
                    ))
                    .unwrap();
                },
            );
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let res: Vec<String> = rx.try_iter().collect();
    assert_eq!(ans, res);

    let n = 5;
    let cars = vec![1, 2, 3, 4, 5];
    let directions = vec![2, 4, 3, 3, 1];
    let arrival_times = vec![10, 20, 30, 40, 40];
    let ans1 = vec_string![
        "Car 1 Has Passed Road A In Direction 2",
        "Traffic Light On Road B Is Green",
        "Car 2 Has Passed Road B In Direction 4",
        "Car 3 Has Passed Road B In Direction 3",
        "Traffic Light On Road A Is Green",
        "Car 5 Has Passed Road A In Direction 1",
        "Traffic Light On Road B Is Green",
        "Car 4 Has Passed Road B In Direction 3"
    ];
    let ans2 = vec_string![
        "Car 1 Has Passed Road A In Direction 2",
        "Traffic Light On Road B Is Green",
        "Car 2 Has Passed Road B In Direction 4",
        "Car 3 Has Passed Road B In Direction 3",
        "Car 4 Has Passed Road B In Direction 3",
        "Traffic Light On Road A Is Green",
        "Car 5 Has Passed Road A In Direction 1"
    ];

    let (tx, rx) = channel();
    let mut threads = vec![];
    let traffic_light = Arc::new(TrafficLight::new());
    for i in 0..n {
        let tx = tx.clone();
        let car_id = cars[i];
        let direction = directions[i];
        let road_id = TrafficLight::road_id(direction);
        let arrival_time = arrival_times[i];
        let traffic_light = traffic_light.clone();
        threads.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(arrival_time));
            traffic_light.car_arrived(
                car_id,
                road_id,
                direction,
                || {
                    tx.send(format!(
                        "Traffic Light On Road {} Is Green",
                        TrafficLight::road_name(road_id)
                    ))
                    .unwrap();
                },
                || {
                    tx.send(format!(
                        "Car {} Has Passed Road {} In Direction {}",
                        car_id,
                        TrafficLight::road_name(road_id),
                        direction
                    ))
                    .unwrap();
                },
            );
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let res: Vec<String> = rx.try_iter().collect();
    assert!(ans1 == res || ans2 == res);
}
