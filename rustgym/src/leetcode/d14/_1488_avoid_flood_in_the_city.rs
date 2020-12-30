struct Solution;
use std::collections::BTreeSet;
use std::collections::HashMap;

impl Solution {
    fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let n = rains.len();
        let mut lakes: HashMap<i32, usize> = HashMap::new();
        let mut dry_days: BTreeSet<usize> = BTreeSet::new();
        let mut res = vec![-1; n];
        for i in 0..n {
            if rains[i] == 0 {
                dry_days.insert(i);
            } else {
                if let Some(j) = lakes.insert(rains[i], i) {
                    let mut pick_a_day: Option<usize> = None;
                    for &k in &dry_days {
                        if j < k {
                            pick_a_day = Some(k);
                            break;
                        }
                    }
                    if let Some(k) = pick_a_day {
                        res[k] = rains[i];
                        dry_days.remove(&k);
                    } else {
                        return vec![];
                    }
                }
            }
        }
        let (any_lake, _) = lakes.into_iter().next().unwrap();
        for i in dry_days {
            res[i] = any_lake;
        }
        res
    }
}

#[test]
fn test() {
    let rains = vec![1, 2, 3, 4];
    let res = vec![-1, -1, -1, -1];
    assert_eq!(Solution::avoid_flood(rains), res);
    let rains = vec![1, 2, 0, 0, 2, 1];
    let res = vec![-1, -1, 2, 1, -1, -1];
    assert_eq!(Solution::avoid_flood(rains), res);
    let rains = vec![1, 2, 0, 1, 2];
    let res: Vec<i32> = vec![];
    assert_eq!(Solution::avoid_flood(rains), res);
    let rains = vec![0, 1, 1];
    let res: Vec<i32> = vec![];
    assert_eq!(Solution::avoid_flood(rains), res);
    let rains = vec![1, 0, 2, 3, 0, 1, 2];
    let res = vec![-1, 1, -1, -1, 2, -1, -1];
    assert_eq!(Solution::avoid_flood(rains), res);
}
