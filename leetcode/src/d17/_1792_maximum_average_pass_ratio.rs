struct Solution;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Debug)]
struct Class {
    pass: i32,
    total: i32,
}

impl Class {
    fn new(pass: i32, total: i32) -> Self {
        Class { pass, total }
    }

    fn add(&mut self) {
        self.pass += 1;
        self.total += 1;
    }

    fn inc(&self) -> f64 {
        (self.pass + 1) as f64 / (self.total + 1) as f64 - self.pass as f64 / self.total as f64
    }
}

impl Ord for Class {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inc().partial_cmp(&other.inc()).unwrap()
    }
}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Class) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let n = classes.len();
        let mut queue: BinaryHeap<Class> = BinaryHeap::new();
        for i in 0..n {
            let class = Class::new(classes[i][0], classes[i][1]);
            queue.push(class);
        }
        for _ in 0..extra_students {
            if let Some(mut top) = queue.pop() {
                top.add();
                queue.push(top);
            }
        }
        let mut sum = 0.0;
        while let Some(top) = queue.pop() {
            sum += top.pass as f64 / top.total as f64;
        }
        sum / n as f64
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let classes = vec_vec_i32![[1, 2], [3, 5], [2, 2]];
    let extra_students = 2;
    let res = 0.78333;
    assert_approx_eq!(
        Solution::max_average_ratio(classes, extra_students),
        res,
        1.0e-5
    );
    let classes = vec_vec_i32![[2, 4], [3, 9], [4, 5], [2, 10]];
    let extra_students = 4;
    let res = 0.53485;
    assert_approx_eq!(
        Solution::max_average_ratio(classes, extra_students),
        res,
        1.0e-5
    );
}
