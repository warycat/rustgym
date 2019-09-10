struct Solution;

impl Solution {
    fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut z = x ^ y;
        let mut sum = 0;
        for _ in 0..32 {
            sum += z & 1;
            z >>= 1;
        }
        sum
    }
}

#[test]
fn test() {
    assert_eq!(Solution::hamming_distance(1, 4), 2);
}
