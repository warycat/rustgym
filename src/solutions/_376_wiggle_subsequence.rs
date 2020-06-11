struct Solution;
use std::cmp::Ordering::*;

impl Solution {
    fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut up: Vec<usize> = vec![1];
        let mut down: Vec<usize> = vec![1];
        for i in 1..n {
            match nums[i].cmp(&nums[i - 1]) {
                Greater => {
                    down.push(up[i - 1] + 1);
                    up.push(up[i - 1]);
                }
                Less => {
                    down.push(down[i - 1]);
                    up.push(down[i - 1] + 1);
                }
                Equal => {
                    down.push(down[i - 1]);
                    up.push(up[i - 1]);
                }
            }
        }
        up.into_iter().chain(down.into_iter()).max().unwrap() as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 7, 4, 9, 2, 5];
    let res = 6;
    assert_eq!(Solution::wiggle_max_length(nums), res);
    let nums = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
    let res = 7;
    assert_eq!(Solution::wiggle_max_length(nums), res);
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let res = 2;
    assert_eq!(Solution::wiggle_max_length(nums), res);
}
