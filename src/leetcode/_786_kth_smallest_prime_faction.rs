struct Solution;
use std::cmp::Ord;
use std::cmp::Ordering;

use std::collections::BinaryHeap;

struct Fraction(i32, i32, usize, usize);

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.0 * other.1 == self.1 * other.0
    }
}

impl Eq for Fraction {}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.1 * other.0).cmp(&(self.0 * other.1))
    }
}

impl Solution {
    fn kth_smallest_prime_fraction(a: Vec<i32>, k: i32) -> Vec<i32> {
        let mut queue: BinaryHeap<Fraction> = BinaryHeap::new();
        let n = a.len();
        let k = k as usize;
        for i in 0..n {
            queue.push(Fraction(a[i], a[n - 1], i, n - 1));
        }
        for _ in 0..k - 1 {
            let f = queue.pop().unwrap();
            if f.3 - 1 > f.2 {
                queue.push(Fraction(a[f.2], a[f.3 - 1], f.2, f.3 - 1));
            }
        }
        let f = queue.pop().unwrap();
        vec![f.0, f.1]
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 3, 5];
    let k = 3;
    let res = vec![2, 5];
    assert_eq!(Solution::kth_smallest_prime_fraction(a, k), res);
    let a = vec![1, 7];
    let k = 1;
    let res = vec![1, 7];
    assert_eq!(Solution::kth_smallest_prime_fraction(a, k), res);
}
