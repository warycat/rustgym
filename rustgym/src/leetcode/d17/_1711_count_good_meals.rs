struct Solution;

use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, i64> = HashMap::new();
        let mut res = 0;
        for x in deliciousness {
            for i in 0..22 {
                let sum = 1 << i;
                let y = sum - x;
                if let Some(k) = hm.get(&y) {
                    res += k;
                    res %= MOD;
                }
            }
            *hm.entry(x).or_default() += 1;
        }
        res as i32
    }
}

#[test]
fn test() {
    let deliciousness = vec![1, 3, 5, 7, 9];
    let res = 4;
    assert_eq!(Solution::count_pairs(deliciousness), res);
    let deliciousness = vec![1, 1, 1, 3, 3, 3, 7];
    let res = 15;
    assert_eq!(Solution::count_pairs(deliciousness), res);
    let deliciousness = vec![
        149, 107, 1, 63, 0, 1, 6867, 1325, 5611, 2581, 39, 89, 46, 18, 12, 20, 22, 234,
    ];
    let res = 12;
    assert_eq!(Solution::count_pairs(deliciousness), res);
}
