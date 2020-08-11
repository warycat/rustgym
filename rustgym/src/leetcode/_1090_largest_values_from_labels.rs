struct Solution;
use std::collections::HashMap;
type Pair = (i32, i32);

impl Solution {
    fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let n = values.len();
        let use_limit = use_limit as usize;
        let mut num_wanted = num_wanted as usize;
        let mut pairs: Vec<Pair> = values.into_iter().zip(labels.into_iter()).collect();
        pairs.sort_unstable();
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut res = 0;
        for i in (0..n).rev() {
            let count = hm.entry(pairs[i].1).or_default();
            if *count < use_limit {
                *count += 1;
                res += pairs[i].0;
                num_wanted -= 1;
            }
            if num_wanted == 0 {
                break;
            }
        }
        res
    }
}

#[test]
fn test() {
    let values = vec![5, 4, 3, 2, 1];
    let labels = vec![1, 1, 2, 2, 3];
    let num_wanted = 3;
    let use_limit = 1;
    let res = 9;
    assert_eq!(
        Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
        res
    );
    let values = vec![5, 4, 3, 2, 1];
    let labels = vec![1, 3, 3, 3, 2];
    let num_wanted = 3;
    let use_limit = 2;
    let res = 12;
    assert_eq!(
        Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
        res
    );
    let values = vec![9, 8, 8, 7, 6];
    let labels = vec![0, 0, 0, 1, 1];
    let num_wanted = 3;
    let use_limit = 1;
    let res = 16;
    assert_eq!(
        Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
        res
    );
    let values = vec![9, 8, 8, 7, 6];
    let labels = vec![0, 0, 0, 1, 1];
    let num_wanted = 3;
    let use_limit = 2;
    let res = 24;
    assert_eq!(
        Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit),
        res
    );
}
