struct Solution;
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    fn min_set_size(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut hm: HashMap<i32, usize> = HashMap::new();
        for x in arr {
            *hm.entry(x).or_default() += 1;
        }
        let mut pq: BinaryHeap<usize> = BinaryHeap::new();
        for (_, v) in hm {
            pq.push(v);
        }
        let mut sum = 0;
        let mut res = 0;
        while sum * 2 < n {
            sum += pq.pop().unwrap();
            res += 1;
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7];
    let res = 2;
    assert_eq!(Solution::min_set_size(arr), res);
    let arr = vec![7, 7, 7, 7, 7, 7];
    let res = 1;
    assert_eq!(Solution::min_set_size(arr), res);
    let arr = vec![1, 9];
    let res = 1;
    assert_eq!(Solution::min_set_size(arr), res);
    let arr = vec![1000, 1000, 3, 7];
    let res = 1;
    assert_eq!(Solution::min_set_size(arr), res);
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let res = 5;
    assert_eq!(Solution::min_set_size(arr), res);
}
