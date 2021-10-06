struct Solution;

impl Solution {
    fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let n = heights.len();
        for i in 0..n {
            while let Some(&j) = stack.last() {
                if heights[j] <= heights[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        stack.into_iter().map(|i| i as i32).collect()
    }
}

#[test]
fn test() {
    let heights = vec![4, 2, 3, 1];
    let res = vec![0, 2, 3];
    assert_eq!(Solution::find_buildings(heights), res);
    let heights = vec![4, 3, 2, 1];
    let res = vec![0, 1, 2, 3];
    assert_eq!(Solution::find_buildings(heights), res);
    let heights = vec![2, 2, 2, 2];
    let res = vec![3];
    assert_eq!(Solution::find_buildings(heights), res);
}
