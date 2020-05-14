struct Solution;
use std::collections::BTreeMap;
use std::collections::VecDeque;

impl Solution {
    fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let k = k as usize;
        if n % k != 0 {
            return false;
        }
        let mut btm: BTreeMap<i32, usize> = BTreeMap::new();
        for x in nums {
            *btm.entry(x).or_default() += 1;
        }
        let mut queue: VecDeque<(i32, usize)> = VecDeque::new();
        for (val, size) in btm {
            queue.push_back((val, size));
            if queue.len() == k {
                let (first_val, first_size) = queue.pop_front().unwrap();
                for i in 1..k {
                    let (next_val, next_size) = queue.pop_front().unwrap();
                    if next_val != first_val + i as i32 {
                        return false;
                    }
                    if next_size < first_size {
                        return false;
                    }
                    if next_size > first_size {
                        queue.push_back((next_val, next_size - first_size));
                    }
                }
            }
        }
        queue.is_empty()
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 3, 4, 4, 5, 6];
    let k = 4;
    let res = true;
    assert_eq!(Solution::is_possible_divide(nums, k), res);
    let nums = vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11];
    let k = 3;
    let res = true;
    assert_eq!(Solution::is_possible_divide(nums, k), res);
    let nums = vec![3, 3, 2, 2, 1, 1];
    let k = 3;
    let res = true;
    assert_eq!(Solution::is_possible_divide(nums, k), res);
    let nums = vec![1, 2, 3, 4];
    let k = 3;
    let res = false;
    assert_eq!(Solution::is_possible_divide(nums, k), res);
}
