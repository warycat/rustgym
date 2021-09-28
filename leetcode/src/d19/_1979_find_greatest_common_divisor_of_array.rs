struct Solution;

impl Solution {
    fn find_gcd(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();
        for i in (2..=*min).rev() {
            if min % i == 0 && max % i == 0 {
                return i;
            }
        }
        1
    }
}

#[test]
fn test() {
    let nums = vec![2, 5, 6, 9, 10];
    let res = 2;
    assert_eq!(Solution::find_gcd(nums), res);
    let nums = vec![7, 5, 6, 8, 3];
    let res = 1;
    assert_eq!(Solution::find_gcd(nums), res);
    let nums = vec![3, 3];
    let res = 3;
    assert_eq!(Solution::find_gcd(nums), res);
}
