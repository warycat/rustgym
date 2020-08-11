struct Solution;

impl Solution {
    fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        let n = pairs.len();
        pairs.sort_by_key(|p| p[1]);
        let mut res = 0;
        let mut cur = std::i32::MIN;
        for i in 0..n {
            if cur < pairs[i][0] {
                cur = pairs[i][1];
                res += 1;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let pairs = vec_vec_i32![[1, 2], [2, 3], [3, 4]];
    let res = 2;
    assert_eq!(Solution::find_longest_chain(pairs), res);
}
