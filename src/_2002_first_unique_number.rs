use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

type Pair = (Reverse<usize>, i32);

#[derive(Default)]
struct FirstUnique {
    counts: HashMap<i32, usize>,
    queue: BinaryHeap<Pair>,
    id: usize,
}

impl FirstUnique {
    fn new(nums: Vec<i32>) -> Self {
        let mut res = FirstUnique::default();
        for x in nums {
            res.add(x);
        }
        res
    }

    fn add(&mut self, value: i32) {
        let count = self.counts.entry(value).or_default();
        *count += 1;
        self.id += 1;
        self.queue.push((Reverse(self.id), value));
    }

    fn show_first_unique(&mut self) -> i32 {
        while let Some(first) = self.queue.pop() {
            let value = first.1;
            if self.counts[&value] == 1 {
                self.queue.push(first);
                return value;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, 5];
    let mut obj = FirstUnique::new(nums);
    assert_eq!(obj.show_first_unique(), 2);
    obj.add(5);
    assert_eq!(obj.show_first_unique(), 2);
    obj.add(2);
    assert_eq!(obj.show_first_unique(), 3);
    obj.add(3);
    assert_eq!(obj.show_first_unique(), -1);

    let nums = vec![7, 7, 7, 7, 7, 7];
    let mut obj = FirstUnique::new(nums);
    assert_eq!(obj.show_first_unique(), -1);
    obj.add(7);
    obj.add(3);
    obj.add(3);
    obj.add(7);
    obj.add(17);
    assert_eq!(obj.show_first_unique(), 17);

    let nums = vec![809];
    let mut obj = FirstUnique::new(nums);
    assert_eq!(obj.show_first_unique(), 809);
    obj.add(809);
    assert_eq!(obj.show_first_unique(), -1);
}
