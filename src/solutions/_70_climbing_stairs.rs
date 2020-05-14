struct Solution;

impl Solution {
    fn climb_stairs(n: i32) -> i32 {
        match n {
            1 | 2 => n,
            k => (2..k).fold((1, 2), |acc, _| (acc.1, acc.0 + acc.1)).1,
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::climb_stairs(3), 3);
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(1), 1);
}
