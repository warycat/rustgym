struct Solution;

impl Solution {
    fn digit_square_sum(mut x: i32) -> i32 {
        let mut sum = 0;
        while x > 0 {
            let tmp = x % 10;
            sum += tmp * tmp;
            x /= 10;
        }
        sum
    }

    fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = n;
        loop {
            slow = Self::digit_square_sum(slow);
            fast = Self::digit_square_sum(fast);
            fast = Self::digit_square_sum(fast);
            if slow == fast {
                break;
            }
        }
        slow == 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_happy(19), true);
}
