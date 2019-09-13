use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    pq: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut pq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let k = k as usize;
        for x in nums {
            pq.push(Reverse(x));
            if pq.len() > k {
                pq.pop();
            }
        }
        KthLargest { pq, k }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.pq.push(Reverse(val));
        if self.pq.len() > self.k {
            self.pq.pop();
        }
        let x = *self.pq.peek().unwrap();
        x.0
    }
}

#[test]
fn test() {
    let k = 3;
    let nums = vec![4, 5, 8, 2];
    let mut obj = KthLargest::new(k, nums);
    assert_eq!(obj.add(3), 4);
    assert_eq!(obj.add(5), 5);
    assert_eq!(obj.add(10), 5);
    assert_eq!(obj.add(9), 8);
    assert_eq!(obj.add(4), 8);
}
