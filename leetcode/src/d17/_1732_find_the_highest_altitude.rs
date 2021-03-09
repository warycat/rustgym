struct Solution;

impl Solution {
    fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut height = 0;
        for x in gain {
            height += x;
            res = res.max(height);
        }
        res
    }
}

#[test]
fn test() {
    let gain = vec![-5, 1, 5, 0, -7];
    let res = 1;
    assert_eq!(Solution::largest_altitude(gain), res);
    let gain = vec![-4, -3, -2, -1, 4, 3, 2];
    let res = 0;
    assert_eq!(Solution::largest_altitude(gain), res);
}
