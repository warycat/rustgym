struct Solution;

impl Solution {
    fn is_good_array(nums: Vec<i32>) -> bool {
        let mut res = nums[0];
        let n = nums.len();
        for i in 0..n {
            res = gcd(res, nums[i]);
            if res == 1 {
                return true;
            }
        }
        false
    }
}

fn gcd(mut m: i32, mut n: i32) -> i32 {
    while m != 0 {
        let temp = m;
        m = n % temp;
        n = temp;
    }
    n.abs()
}

#[test]
fn test() {
    let nums = vec![12, 5, 7, 23];
    let res = true;
    assert_eq!(Solution::is_good_array(nums), res);
    let nums = vec![29, 6, 10];
    let res = true;
    assert_eq!(Solution::is_good_array(nums), res);
    let nums = vec![3, 6];
    let res = false;
    assert_eq!(Solution::is_good_array(nums), res);
}
