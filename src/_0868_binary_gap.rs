struct Solution;

impl Solution {
    fn binary_gap(n: i32) -> i32 {
        let mut max = 0;
        let mut prev: Option<usize> = None;
        for i in 0..32 {
            let bit = 1 << i;
            if n & bit != 0 {
                if let Some(j) = prev {
                    max = usize::max(i - j, max);
                }
                prev = Some(i);
            }
        }
        max as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::binary_gap(22), 2);
    assert_eq!(Solution::binary_gap(5), 2);
    assert_eq!(Solution::binary_gap(6), 1);
    assert_eq!(Solution::binary_gap(8), 0);
}
