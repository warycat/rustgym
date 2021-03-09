struct Solution;

use std::collections::HashMap;

impl Solution {
    fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut count: HashMap<i32, usize> = HashMap::new();
        let mut max = 0;
        for rect in rectangles {
            let min = rect[0].min(rect[1]);
            *count.entry(min).or_default() += 1;
            max = max.max(min);
        }
        count[&max] as i32
    }
}

#[test]
fn test() {
    let rectangles = vec_vec_i32![[5, 8], [3, 9], [5, 12], [16, 5]];
    let res = 3;
    assert_eq!(Solution::count_good_rectangles(rectangles), res);
    let rectangles = vec_vec_i32![[2, 3], [3, 7], [4, 3], [3, 7]];
    let res = 3;
    assert_eq!(Solution::count_good_rectangles(rectangles), res);
}
