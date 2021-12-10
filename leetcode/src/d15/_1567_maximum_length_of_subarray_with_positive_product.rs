struct Solution;

impl Solution {
    fn get_max_len(nums: Vec<i32>) -> i32 {
        nums.split(|&x| x == 0)
            .map(Self::max_length)
            .max()
            .unwrap_or(0)
    }
    fn max_length(v: &[i32]) -> i32 {
        let mut neg = 0;
        let mut res = 0;
        let n = v.len();
        let mut first_neg: Option<usize> = None;
        for i in 0..n {
            if v[i] < 0 {
                neg += 1;
                if first_neg.is_none() {
                    first_neg = Some(i);
                }
            }
            if neg % 2 == 0 {
                res = res.max((i + 1) as i32);
            } else {
                if let Some(j) = first_neg {
                    res = res.max((i - j) as i32);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, -2, -3, 4];
    let res = 4;
    assert_eq!(Solution::get_max_len(nums), res);
    let nums = vec![0, 1, -2, -3, -4];
    let res = 3;
    assert_eq!(Solution::get_max_len(nums), res);
    let nums = vec![-1, -2, -3, 0, 1];
    let res = 2;
    assert_eq!(Solution::get_max_len(nums), res);
    let nums = vec![-1, 2];
    let res = 1;
    assert_eq!(Solution::get_max_len(nums), res);
    let nums = vec![1, 2, 3, 5, -6, 4, 0, 10];
    let res = 4;
    assert_eq!(Solution::get_max_len(nums), res);
}
