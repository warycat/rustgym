struct Solution;

use std::collections::HashMap;

impl Solution {
    fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let n = n as usize;
        let m = tasks.len();
        let mut hm: HashMap<char, usize> = HashMap::new();
        let mut max = 0;
        for t in tasks {
            let count = hm.entry(t).or_default();
            *count += 1;
            max = usize::max(*count, max);
        }
        let v = hm.iter().fold(
            0,
            |sum, (_, &count)| if count == max { sum + 1 } else { sum },
        );

        usize::max(m, (n + 1) * (max - 1) + v) as i32
    }
}

#[test]
fn test() {
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    assert_eq!(Solution::least_interval(tasks, 2), 8);
}
