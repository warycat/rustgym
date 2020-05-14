struct Solution;

impl Solution {
    fn digits(mut n: i32) -> i32 {
        let mut res = 0;
        while n != 0 {
            n /= 10;
            res += 1;
        }
        res
    }
    fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for n in nums {
            if Self::digits(n) % 2 == 0 {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
}
