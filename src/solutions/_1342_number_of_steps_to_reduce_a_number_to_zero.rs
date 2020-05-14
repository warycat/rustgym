struct Solution;

impl Solution {
    fn number_of_steps(mut num: i32) -> i32 {
        let mut res = 0;
        while num != 0 {
            if num & 1 == 1 {
                num -= 1;
            } else {
                num >>= 1;
            }
            res += 1;
        }
        res
    }
}

#[test]
fn test() {
    let num = 14;
    let res = 6;
    assert_eq!(Solution::number_of_steps(num), res);
    let num = 8;
    let res = 4;
    assert_eq!(Solution::number_of_steps(num), res);
    let num = 123;
    let res = 12;
    assert_eq!(Solution::number_of_steps(num), res);
}
