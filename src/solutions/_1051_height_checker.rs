struct Solution;

impl Solution {
    fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sorted = heights.to_vec();
        sorted.sort_unstable();
        heights
            .iter()
            .zip(sorted.iter())
            .fold(0, |sum, (a, b)| if a != b { sum + 1 } else { sum })
    }
}

#[test]
fn test() {
    let heights = vec![1, 1, 4, 2, 1, 3];
    assert_eq!(Solution::height_checker(heights), 3);
}
