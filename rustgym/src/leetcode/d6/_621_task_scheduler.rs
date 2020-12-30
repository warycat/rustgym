struct Solution;

use std::collections::HashMap;

impl Solution {
    fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let n = n as usize;
        let m = tasks.len();
        let mut hm: HashMap<char, usize> = HashMap::new();
        let mut max_count = 0;
        for t in tasks {
            let count = hm.entry(t).or_default();
            *count += 1;
            max_count = max_count.max(*count);
        }
        let task_with_max_count = hm.values().filter(|&&x| x == max_count).count();
        usize::max(m, (n + 1) * (max_count - 1) + task_with_max_count) as i32
    }
}

#[test]
fn test() {
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    let n = 2;
    let res = 8;
    assert_eq!(Solution::least_interval(tasks, n), res);
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    let n = 0;
    let res = 6;
    assert_eq!(Solution::least_interval(tasks, n), res);
    let tasks = vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'];
    let n = 2;
    let res = 16;
    assert_eq!(Solution::least_interval(tasks, n), res);
}
