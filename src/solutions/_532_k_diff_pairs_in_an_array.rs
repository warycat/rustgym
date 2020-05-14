struct Solution;

use std::cmp::Ordering::*;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut hs: HashSet<i32> = HashSet::new();
        let mut hm: HashMap<i32, i32> = HashMap::new();
        match k.cmp(&0) {
            Equal => {
                for x in nums {
                    let e = hm.entry(x).or_default();
                    if let 1 = *e {
                        res += 1;
                    }
                    *e += 1;
                }
            }
            Greater => {
                for x in nums {
                    if hs.contains(&x) {
                        continue;
                    } else {
                        hs.insert(x);
                        if hs.contains(&(x + k)) {
                            res += 1;
                        }
                        if hs.contains(&(x - k)) {
                            res += 1;
                        }
                    }
                }
            }
            Less => {}
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![3, 1, 4, 1, 5];
    let k = 2;
    assert_eq!(Solution::find_pairs(nums, k), 2);
    let nums = vec![1, 2, 3, 4, 5];
    let k = 1;
    assert_eq!(Solution::find_pairs(nums, k), 4);
    let nums = vec![1, 3, 1, 5, 4];
    let k = 0;
    assert_eq!(Solution::find_pairs(nums, k), 1);
    let nums = vec![1, 1, 1, 1, 1];
    let k = 0;
    assert_eq!(Solution::find_pairs(nums, k), 1);
}
