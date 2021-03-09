struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn min_k_bit_flips(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let k = k as usize;
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut res = 0;
        for i in 0..n {
            if let Some(&j) = queue.front() {
                if j + k == i {
                    queue.pop_front().unwrap();
                }
            }
            if i + k <= n {
                if (queue.len() as i32 + a[i]) % 2 == 0 {
                    queue.push_back(i);
                    res += 1;
                }
            } else {
                if (queue.len() as i32 + a[i]) % 2 == 0 {
                    return -1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![0, 1, 0];
    let k = 1;
    let res = 2;
    assert_eq!(Solution::min_k_bit_flips(a, k), res);
    let a = vec![1, 1, 0];
    let k = 2;
    let res = -1;
    assert_eq!(Solution::min_k_bit_flips(a, k), res);
    let a = vec![0, 0, 0, 1, 0, 1, 1, 0];
    let k = 3;
    let res = 3;
    assert_eq!(Solution::min_k_bit_flips(a, k), res);
}
