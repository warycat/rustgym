struct Solution;
use std::collections::HashMap;

impl Solution {
    fn count_largest_group(n: i32) -> i32 {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut max = 0;
        for i in 1..=n {
            let mut sum = 0;
            let mut n = i;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }
            let count = hm.entry(sum).or_default();
            *count += 1;
            max = max.max(*count);
        }
        hm.values().filter(|&&v| v == max).count() as i32
    }
}

#[test]
fn test() {
    let n = 13;
    let res = 4;
    assert_eq!(Solution::count_largest_group(n), res);
    let n = 2;
    let res = 2;
    assert_eq!(Solution::count_largest_group(n), res);
    let n = 15;
    let res = 6;
    assert_eq!(Solution::count_largest_group(n), res);
    let n = 24;
    let res = 5;
    assert_eq!(Solution::count_largest_group(n), res);
}
