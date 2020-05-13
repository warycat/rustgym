struct Solution;

impl Solution {
    fn single_number(nums: Vec<i32>) -> i32 {
        let mut once = 0;
        let mut twice = 0;
        for x in nums {
            once = !twice & (once ^ x);
            twice = !once & (twice ^ x);
        }
        once
    }
}

#[test]
fn test() {
    let nums = vec![2, 2, 3, 2];
    let res = 3;
    assert_eq!(Solution::single_number(nums), res);
    let nums = vec![0, 1, 0, 1, 0, 1, 99];
    let res = 99;
    assert_eq!(Solution::single_number(nums), res);
}
