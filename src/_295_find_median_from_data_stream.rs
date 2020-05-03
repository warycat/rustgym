use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
struct MedianFinder {
    lo: BinaryHeap<i32>,
    hi: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder::default()
    }

    fn add_num(&mut self, num: i32) {
        self.hi.push(Reverse(num));
        let smallest = self.hi.pop().unwrap().0;
        self.lo.push(smallest);
        if self.lo.len() > self.hi.len() + 1 {
            self.hi.push(Reverse(self.lo.pop().unwrap()));
        }
    }

    fn find_median(&self) -> f64 {
        if (self.lo.len() + self.hi.len()) % 2 == 0 {
            (*self.lo.peek().unwrap() + self.hi.peek().unwrap().0) as f64 / 2.0
        } else {
            *self.lo.peek().unwrap() as f64
        }
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let mut obj = MedianFinder::new();
    obj.add_num(1);
    obj.add_num(2);
    assert_approx_eq!(obj.find_median(), 1.5);
    obj.add_num(3);
    assert_approx_eq!(obj.find_median(), 2.0);
}
