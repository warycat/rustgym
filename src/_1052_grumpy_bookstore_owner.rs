struct Solution;

use std::i32;

impl Solution {
    fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let mut sum = 0;
        let mut max = 0;
        let n = customers.len();
        for i in 0..n {
            if grumpy[i] == 0 {
                sum += customers[i];
            }
        }
        let x = x as usize;
        for i in 0..x {
            if grumpy[i] == 1 {
                sum += customers[i];
            }
        }
        max = i32::max(sum, max);
        for i in x..n {
            if grumpy[i] == 1 {
                sum += customers[i];
            }
            if grumpy[i - x] == 1 {
                sum -= customers[i - x];
            }
            max = i32::max(sum, max);
        }
        max
    }
}

#[test]
fn test() {
    let customers = vec![1, 0, 1, 2, 1, 1, 7, 5];
    let grumpy = vec![0, 1, 0, 1, 0, 1, 0, 1];
    let x = 3;
    let res = 16;
    assert_eq!(Solution::max_satisfied(customers, grumpy, x), res);
}
