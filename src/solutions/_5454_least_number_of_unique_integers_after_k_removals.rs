struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut k = k as usize;
        let mut count: HashMap<i32, usize> = HashMap::new();
        for x in arr {
            *count.entry(x).or_default() += 1;
        }
        let mut queue: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
        for (_, v) in count {
            queue.push(Reverse(v));
        }
        while let Some(&min) = queue.peek() {
            if min.0 <= k {
                k -= min.0;
                queue.pop();
            } else {
                break;
            }
        }
        queue.len() as i32
    }
}

#[test]
fn test() {
    let arr = vec![5, 5, 4];
    let k = 1;
    let res = 1;
    assert_eq!(Solution::find_least_num_of_unique_ints(arr, k), res);
    let arr = vec![4, 3, 1, 1, 3, 3, 2];
    let k = 3;
    let res = 2;
    assert_eq!(Solution::find_least_num_of_unique_ints(arr, k), res);
}
