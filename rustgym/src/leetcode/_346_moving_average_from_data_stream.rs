use std::collections::VecDeque;

struct MovingAverage {
    queue: VecDeque<i32>,
    sum: i32,
    size: usize,
}

impl MovingAverage {
    fn new(size: i32) -> Self {
        MovingAverage {
            queue: VecDeque::new(),
            sum: 0,
            size: size as usize,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.sum += val;
        self.queue.push_back(val);
        if self.queue.len() > self.size {
            if let Some(front) = self.queue.pop_front() {
                self.sum -= front;
            }
        }
        self.sum as f64 / self.queue.len() as f64
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let mut m = MovingAverage::new(3);
    assert_approx_eq!(m.next(1), 1f64);
    assert_approx_eq!(m.next(10), 5.5f64);
    assert_approx_eq!(m.next(3), 14f64 / 3f64);
    assert_approx_eq!(m.next(5), 6f64);
}
