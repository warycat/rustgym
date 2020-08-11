struct Solution;
use std::collections::HashMap;

impl Solution {
    fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let n = wall.len();
        let w: i32 = wall[0].iter().sum();
        let mut hm: HashMap<i32, usize> = HashMap::new();
        for row in wall {
            let mut sum = 0;
            for x in row {
                sum += x;
                *hm.entry(sum).or_default() += 1;
            }
        }
        let mut max = 0;
        for (k, v) in hm {
            if k != w {
                max = max.max(v);
            }
        }
        (n - max) as i32
    }
}

#[test]
fn test() {
    let wall = vec_vec_i32![
        [1, 2, 2, 1],
        [3, 1, 2],
        [1, 3, 2],
        [2, 4],
        [3, 1, 2],
        [1, 3, 1, 1]
    ];
    let res = 2;
    assert_eq!(Solution::least_bricks(wall), res);
}
