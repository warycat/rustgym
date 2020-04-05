struct Solution;
use std::collections::HashMap;

impl Solution {
    fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for x in arr {
            *hm.entry(x).or_default() += 1;
        }
        if let Some(x) = hm
            .into_iter()
            .filter_map(|(k, v)| if k == v { Some(k) } else { None })
            .max()
        {
            x
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    let arr = vec![2, 2, 3, 4];
    let res = 2;
    assert_eq!(Solution::find_lucky(arr), res);
    let arr = vec![1, 2, 2, 3, 3, 3];
    let res = 3;
    assert_eq!(Solution::find_lucky(arr), res);
    let arr = vec![2, 2, 2, 3, 3];
    let res = -1;
    assert_eq!(Solution::find_lucky(arr), res);
    let arr = vec![5];
    let res = -1;
    assert_eq!(Solution::find_lucky(arr), res);
    let arr = vec![7, 7, 7, 7, 7, 7, 7];
    let res = 7;
    assert_eq!(Solution::find_lucky(arr), res);
}
