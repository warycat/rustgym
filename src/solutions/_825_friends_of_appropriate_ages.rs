struct Solution;
use std::collections::HashMap;

impl Solution {
    fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for age in ages {
            *hm.entry(age).or_default() += 1;
        }
        let mut res = 0;
        for (&a, v) in &hm {
            for (&b, u) in &hm {
                if !(b > a || 2 * b <= a + 14) {
                    res += v * u;
                    if a == b {
                        res -= v;
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let ages = vec![16, 16];
    let res = 2;
    assert_eq!(Solution::num_friend_requests(ages), res);
    let ages = vec![16, 17, 18];
    let res = 2;
    assert_eq!(Solution::num_friend_requests(ages), res);
    let ages = vec![20, 30, 100, 110, 120];
    let res = 3;
    assert_eq!(Solution::num_friend_requests(ages), res);
    let ages = vec![73, 106, 39, 6, 26, 15, 30, 100, 71, 35, 46, 112, 6, 60, 110];
    let res = 29;
    assert_eq!(Solution::num_friend_requests(ages), res);
}
