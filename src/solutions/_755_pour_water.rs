struct Solution;

impl Solution {
    fn pour_water(mut heights: Vec<i32>, v: i32, k: i32) -> Vec<i32> {
        for _ in 0..v {
            let mut cur = k as usize;
            while cur > 0 && heights[cur] >= heights[cur - 1] {
                cur -= 1;
            }
            while cur < heights.len() - 1 && heights[cur] >= heights[cur + 1] {
                cur += 1;
            }
            while cur > k as usize && heights[cur] >= heights[cur - 1] {
                cur -= 1;
            }
            heights[cur] += 1;
        }
        heights
    }
}

#[test]
fn test() {
    let heights = vec![2, 1, 1, 2, 1, 2, 2];
    let v = 4;
    let k = 3;
    let output = vec![2, 2, 2, 3, 2, 2, 2];
    assert_eq!(Solution::pour_water(heights, v, k), output);

    let heights = vec![1, 2, 3, 4];
    let v = 2;
    let k = 2;
    let output = vec![2, 3, 3, 4];
    assert_eq!(Solution::pour_water(heights, v, k), output);

    let heights = vec![3, 1, 3];
    let v = 5;
    let k = 1;
    let output = vec![4, 4, 4];
    assert_eq!(Solution::pour_water(heights, v, k), output);
}
