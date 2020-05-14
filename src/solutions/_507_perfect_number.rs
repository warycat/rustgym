struct Solution;

impl Solution {
    fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        let mut i = 2;
        let mut sum = 1;
        while i * i <= num {
            if num % i == 0 {
                sum += i;
                sum += num / i;
            }
            i += 1;
        }
        sum == num
    }
}

#[test]
fn test() {
    assert_eq!(Solution::check_perfect_number(28), true);
}
