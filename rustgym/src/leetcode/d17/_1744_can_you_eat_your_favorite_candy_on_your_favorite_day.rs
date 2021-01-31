struct Solution;

impl Solution {
    fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut prefix: Vec<i64> = vec![0];
        let mut prev = 0;
        for count in candies_count {
            prev += count as i64;
            prefix.push(prev);
        }
        let mut res = vec![];
        for q in queries {
            let t = q[0] as usize;
            let d = q[1] as i64;
            let c = q[2] as i64;
            res.push(prefix[t] / c <= d && d < prefix[t + 1]);
        }

        res
    }
}

#[test]
fn test() {
    let candies_count = vec![7, 4, 5, 3, 8];
    let queries = vec_vec_i32![[0, 2, 2], [4, 2, 4], [2, 13, 1000000000]];
    let res = vec![true, false, true];
    assert_eq!(Solution::can_eat(candies_count, queries), res);
    let candies_count = vec![5, 2, 6, 4, 1];
    let queries = vec_vec_i32![[3, 1, 2], [4, 10, 3], [3, 10, 100], [4, 100, 30], [1, 3, 1]];
    let res = vec![false, true, true, false, false];
    assert_eq!(Solution::can_eat(candies_count, queries), res);
    let candies_count = vec![7, 11, 5, 3, 8];
    let queries = vec_vec_i32![[2, 2, 6], [4, 2, 4], [2, 13, 1000000000]];
    let res = vec![false, false, true];
    assert_eq!(Solution::can_eat(candies_count, queries), res);
}
