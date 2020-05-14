use std::collections::VecDeque;

#[derive(Default)]
struct HitCounter {
    queue: VecDeque<i32>,
}

impl HitCounter {
    fn new() -> Self {
        HitCounter {
            queue: VecDeque::new(),
        }
    }

    fn hit(&mut self, timestamp: i32) {
        while let Some(first) = self.queue.front() {
            if first + 300 <= timestamp {
                self.queue.pop_front();
            } else {
                break;
            }
        }
        self.queue.push_back(timestamp);
    }

    fn get_hits(&mut self, timestamp: i32) -> i32 {
        while let Some(first) = self.queue.front() {
            if first + 300 <= timestamp {
                self.queue.pop_front();
            } else {
                break;
            }
        }
        self.queue.len() as i32
    }
}

#[test]
fn test() {
    let mut obj = HitCounter::new();
    obj.hit(1);
    obj.hit(2);
    obj.hit(3);
    assert_eq!(obj.get_hits(4), 3);
    obj.hit(300);
    assert_eq!(obj.get_hits(300), 4);
    assert_eq!(obj.get_hits(301), 3);
}
