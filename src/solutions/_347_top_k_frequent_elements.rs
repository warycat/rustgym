struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

type Pair = (Reverse<usize>, i32);

impl Solution {
    fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut pq: BinaryHeap<Pair> = BinaryHeap::new();
        for x in nums {
            *hm.entry(x).or_default() += 1;
        }
        for (x, f) in hm {
            pq.push((Reverse(f), x));
            if pq.len() > k {
                pq.pop();
            }
        }
        pq.into_iter().map(|p| p.1).rev().collect()
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let res = vec![1, 2];
    assert_eq!(Solution::top_k_frequent(nums, k), res);
}
