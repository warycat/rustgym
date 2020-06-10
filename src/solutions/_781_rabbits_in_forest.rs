struct Solution;
use std::collections::HashMap;

impl Solution {
    fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for x in answers {
            *hm.entry(x).or_default() += 1;
        }
        let mut res = 0;
        for (k, v) in hm {
            res += (v + k) / (k + 1) * (k + 1);
        }
        res
    }
}

#[test]
fn test() {
    let answers = vec![1, 1, 2];
    let res = 5;
    assert_eq!(Solution::num_rabbits(answers), res);
    let answers = vec![10, 10, 10];
    let res = 11;
    assert_eq!(Solution::num_rabbits(answers), res);
    let answers = vec![];
    let res = 0;
    assert_eq!(Solution::num_rabbits(answers), res);
}
