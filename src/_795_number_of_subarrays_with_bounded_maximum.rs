struct Solution;

impl Solution {
    fn num_subarray_bounded_max(a: Vec<i32>, l: i32, r: i32) -> i32 {
        let n = a.len();
        let mut start = 0;
        let mut end = 0;
        let mut res = 0;
        for i in 0..n {
            if a[i] > r {
                start = i + 1;
            }
            if a[i] >= l {
                end = i + 1;
            }
            if start < end {
                res += end - start;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let a = vec![2, 1, 4, 3];
    let l = 2;
    let r = 3;
    let res = 3;
    assert_eq!(Solution::num_subarray_bounded_max(a, l, r), res);
}
