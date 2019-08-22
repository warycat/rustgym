struct Solution;

impl Solution {
    fn add_digits(mut num: i32) -> i32 {
        while num > 9 {
            num = Self::add_digits_once(num);
        }
        num
    }
    fn add_digits_once(mut num: i32) -> i32 {
        let mut res = 0;
        while num > 0 {
            res += num % 10;
            num /= 10;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::add_digits(38), 2);
}
