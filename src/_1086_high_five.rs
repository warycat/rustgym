struct Solution;
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;

impl Solution {
    fn high_five(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut btm: BTreeMap<i32, (BinaryHeap<Reverse<i32>>, i32)> = BTreeMap::new();
        let mut res = vec![];
        for item in items {
            let id = item[0];
            let score = item[1];
            let (pq, sum) = btm.entry(id).or_default();
            pq.push(Reverse(score));
            *sum += score;
            if pq.len() > 5 {
                *sum -= pq.pop().unwrap().0;
            }
        }
        for (id, (_, sum)) in btm {
            res.push(vec![id, sum / 5]);
        }
        res
    }
}

#[test]
fn test() {
    let items = vec_vec_i32![
        [1, 91],
        [1, 92],
        [2, 93],
        [2, 97],
        [1, 60],
        [2, 77],
        [1, 65],
        [1, 87],
        [1, 100],
        [2, 100],
        [2, 76]
    ];
    let res = vec_vec_i32![[1, 87], [2, 88]];
    assert_eq!(Solution::high_five(items), res);
}
