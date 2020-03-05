struct Solution;

impl Solution {
    fn lexical_order(n: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 1..10 {
            Self::dfs(i, &mut res, n);
        }
        res
    }
    fn dfs(cur: i32, all: &mut Vec<i32>, n: i32) {
        if cur > n {
            return;
        }
        all.push(cur);
        for i in 0..10 {
            Self::dfs(cur * 10 + i, all, n);
        }
    }
}

#[test]
fn test() {
    let n = 13;
    let res = vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(Solution::lexical_order(n), res);
}
