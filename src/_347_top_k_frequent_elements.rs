struct Solution;
use std::collections::HashMap;
struct Pair {
    x: i32,
    f: usize,
}

impl Solution {
    fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut pairs: Vec<Pair> = vec![];
        for x in nums {
            *hm.entry(x).or_default() += 1;
        }
        for (x, f) in hm {
            pairs.push(Pair { x, f })
        }
        pairs.sort_unstable_by_key(|p| p.f);
        pairs.iter().map(|p| p.x).rev().take(k).collect()
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let res = vec![1, 2];
    assert_eq!(Solution::top_k_frequent(nums, k), res);
}
