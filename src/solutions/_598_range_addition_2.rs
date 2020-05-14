struct Solution;

impl Solution {
    fn max_count(mut m: i32, mut n: i32, ops: Vec<Vec<i32>>) -> i32 {
        for op in ops {
            m = i32::min(op[0], m);
            n = i32::min(op[1], n);
        }
        m * n
    }
}

#[test]
fn test() {
    let m = 3;
    let n = 3;
    let ops: Vec<Vec<i32>> = vec_vec_i32![[2, 2], [3, 3]];
    let res = 4;
    assert_eq!(Solution::max_count(m, n, ops), res);
}
