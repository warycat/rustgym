struct Solution;

use std::cmp::Reverse;

type People = (Reverse<i32>, i32);

impl Solution {
    fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people: Vec<People> = people.iter().map(|v| (Reverse(v[0]), v[1])).collect();
        people.sort_unstable();
        let mut res = vec![];
        for p in people {
            res.insert(p.1 as usize, vec![(p.0).0, p.1]);
        }
        res
    }
}

#[test]
fn test() {
    let people = vec_vec_i32![[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]];
    let res = vec![[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]];
    assert_eq!(Solution::reconstruct_queue(people), res);
}
