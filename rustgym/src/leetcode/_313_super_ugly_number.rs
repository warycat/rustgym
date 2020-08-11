struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    fn nth_super_ugly_number(mut n: i32, primes: Vec<i32>) -> i32 {
        let mut queue: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut visited: HashSet<i32> = HashSet::new();
        queue.push(Reverse(1));
        while n > 1 {
            let min = queue.pop().unwrap().0;
            for &x in &primes {
                if let Some(y) = x.checked_mul(min) {
                    if visited.insert(y) {
                        queue.push(Reverse(y));
                    }
                }
            }
            n -= 1;
        }
        queue.pop().unwrap().0
    }
}

#[test]
fn test() {
    let n = 12;
    let primes = vec![2, 7, 13, 19];
    let res = 32;
    assert_eq!(Solution::nth_super_ugly_number(n, primes), res);
}
