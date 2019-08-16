struct Solution;

struct Degree {
    left: usize,
    right: usize,
    count: usize,
}

use std::collections::HashMap;
use std::usize;

impl Solution {
    fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, Degree> = HashMap::new();
        let n = nums.len();
        let mut max_degree: usize = 0;
        for i in 0..n {
            let x = nums[i];
            let e = hm.entry(x).or_insert(Degree {
                left: i,
                right: i,
                count: 0,
            });
            e.left = usize::min(e.left, i);
            e.right = usize::max(e.right, i);
            e.count += 1;
            max_degree = usize::max(e.count, max_degree);
        }
        let mut min_width: usize = n;
        for d in hm.values() {
            if d.count == max_degree {
                min_width = usize::min(d.right - d.left + 1, min_width);
            }
        }
        min_width as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 2, 3, 1];
    assert_eq!(Solution::find_shortest_sub_array(nums), 2);
    let nums = vec![1, 2, 2, 3, 1, 4, 2];
    assert_eq!(Solution::find_shortest_sub_array(nums), 6);
}
