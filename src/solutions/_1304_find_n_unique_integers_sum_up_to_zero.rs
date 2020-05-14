struct Solution;

impl Solution {
    fn sum_zero(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res: Vec<i32> = vec![0; n];
        let mut sum: i32 = 0;
        for i in 0..n - 1 {
            sum -= i as i32;
            res[i] = i as i32;
        }
        res[n - 1] = sum;
        res
    }
}

#[test]
fn test() {
    let s: i32 = Solution::sum_zero(5).iter().sum();
    assert_eq!(s, 0);
    let s: i32 = Solution::sum_zero(3).iter().sum();
    assert_eq!(s, 0);
    let s: i32 = Solution::sum_zero(1).iter().sum();
    assert_eq!(s, 0);
}
