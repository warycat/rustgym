struct Solution;

impl Solution {
    fn num_times_all_blue(light: Vec<i32>) -> i32 {
        let n = light.len();
        let mut res = 0;
        let mut max = std::usize::MIN;
        for i in 0..n {
            max = max.max((light[i] - 1) as usize);
            if max == i {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let light = vec![2, 1, 3, 5, 4];
    let res = 3;
    assert_eq!(Solution::num_times_all_blue(light), res);
    let light = vec![3, 2, 4, 1, 5];
    let res = 2;
    assert_eq!(Solution::num_times_all_blue(light), res);
    let light = vec![4, 1, 2, 3];
    let res = 1;
    assert_eq!(Solution::num_times_all_blue(light), res);
    let light = vec![2, 1, 4, 3, 6, 5];
    let res = 3;
    assert_eq!(Solution::num_times_all_blue(light), res);
    let light = vec![1, 2, 3, 4, 5, 6];
    let res = 6;
    assert_eq!(Solution::num_times_all_blue(light), res);
}
