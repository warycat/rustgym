struct Solution;

use std::collections::BTreeMap;

impl Solution {
    fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut btm: BTreeMap<i32, f64> = BTreeMap::new();
        let n = position.len();
        for i in 0..n {
            btm.insert(
                target - position[i],
                (target - position[i]) as f64 / speed[i] as f64,
            );
        }
        let mut res = 0;
        let mut cur = 0.0;
        for (_, t) in btm {
            if t > cur {
                res += 1;
                cur = t;
            }
        }
        res
    }
}

#[test]
fn test() {
    let target = 12;
    let position = vec![10, 8, 0, 5, 3];
    let speed = vec![2, 4, 1, 1, 3];
    let res = 3;
    assert_eq!(Solution::car_fleet(target, position, speed), res);
}
