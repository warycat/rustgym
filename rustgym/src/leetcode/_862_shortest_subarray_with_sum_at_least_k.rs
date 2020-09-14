struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn shortest_subarray(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut prefix = vec![0; n + 1];
        queue.push_back(0);
        let mut res = std::usize::MAX;
        for i in 0..n {
            prefix[i + 1] = prefix[i] + a[i];
            while let Some(&j) = queue.front() {
                if prefix[i + 1] - prefix[j] >= k {
                    res = res.min(i + 1 - j);
                    queue.pop_front();
                } else {
                    break;
                }
            }
            while let Some(&j) = queue.back() {
                if prefix[i + 1] <= prefix[j] {
                    queue.pop_back();
                } else {
                    break;
                }
            }
            queue.push_back(i + 1);
        }
        if res == std::usize::MAX {
            -1
        } else {
            res as i32
        }
    }
}

#[test]
fn test() {
    let a = vec![1];
    let k = 1;
    let res = 1;
    assert_eq!(Solution::shortest_subarray(a, k), res);
    let a = vec![1, 2];
    let k = 4;
    let res = -1;
    assert_eq!(Solution::shortest_subarray(a, k), res);
    let a = vec![2, -1, 2];
    let k = 3;
    let res = 3;
    assert_eq!(Solution::shortest_subarray(a, k), res);
}
