struct Solution;

impl Solution {
    fn reach_number(target: i32) -> i32 {
        let target = target.abs();
        let n = (((2 * target as i64) as f64 + 0.25).sqrt() - 0.5).ceil() as i32;
        let sum = n * (n + 1) / 2;
        if sum == target {
            n
        } else {
            let diff = sum - target;
            if diff % 2 == 0 {
                n
            } else {
                if n % 2 == 0 {
                    n + 1
                } else {
                    n + 2
                }
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reach_number(3), 2);
    assert_eq!(Solution::reach_number(2), 3);
    assert_eq!(Solution::reach_number(5), 5);
}
