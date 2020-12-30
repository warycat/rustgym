struct Solution;

impl Solution {
    fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut l = 0;
        let mut r = height.len() - 1;
        while l < r {
            max = i32::max(i32::min(height[l], height[r]) * (r - l) as i32, max);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        max
    }
}

#[test]
fn test() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(Solution::max_area(height), 49);
}
