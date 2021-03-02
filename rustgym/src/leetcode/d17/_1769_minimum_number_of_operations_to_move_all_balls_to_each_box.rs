struct Solution;

impl Solution {
    fn min_operations(boxes: String) -> Vec<i32> {
        let n = boxes.len();
        let boxes: Vec<bool> = boxes.chars().map(|c| c == '1').collect();
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        let mut prev = 0;

        for i in 0..n {
            left[i] += prev;
            if i > 0 {
                left[i] += left[i - 1];
            }
            if boxes[i] {
                prev += 1;
            }
        }
        prev = 0;
        for i in (0..n).rev() {
            right[i] += prev;
            if i + 1 < n {
                right[i] += right[i + 1];
            }
            if boxes[i] {
                prev += 1;
            }
        }
        let mut res = vec![0; n];
        for i in 0..n {
            res[i] = left[i] + right[i];
        }
        res
    }
}

#[test]
fn test() {
    let boxes = "110".to_string();
    let res = vec![1, 1, 3];
    assert_eq!(Solution::min_operations(boxes), res);
    let boxes = "001011".to_string();
    let res = vec![11, 8, 5, 4, 3, 4];
    assert_eq!(Solution::min_operations(boxes), res);
}
