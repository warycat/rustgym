struct Solution;

impl Solution {
    fn longest_ones(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let mut sum = 0;
        let mut res = 0;
        let mut start = 0;
        let mut end = 0;
        while end < n {
            if sum <= k {
                if a[end] == 0 {
                    sum += 1;
                }
                end += 1;
            } else {
                if a[start] == 0 {
                    sum -= 1;
                }
                start += 1;
            }
            if sum <= k {
                res = res.max(end - start);
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let a = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
    let k = 2;
    let res = 6;
    assert_eq!(Solution::longest_ones(a, k), res);
    let a = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
    let k = 3;
    let res = 10;
    assert_eq!(Solution::longest_ones(a, k), res);
    let a = vec![0, 0, 0, 1];
    let k = 4;
    let res = 4;
    assert_eq!(Solution::longest_ones(a, k), res);
}
