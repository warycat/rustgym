struct Solution;

use std::collections::HashMap;

impl Solution {
    fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let n = boxes.len();
        let mut memo: HashMap<(usize, usize, usize), usize> = HashMap::new();
        Self::dp(0, n, 0, &mut memo, &boxes) as i32
    }

    fn dp(
        mut start: usize,
        end: usize,
        mut k: usize,
        memo: &mut HashMap<(usize, usize, usize), usize>,
        boxes: &[i32],
    ) -> usize {
        if start == end {
            return 0;
        }
        if let Some(&res) = memo.get(&(start, end, k)) {
            return res;
        }
        while start + 1 < end && boxes[start + 1] == boxes[start] {
            start += 1;
            k += 1;
        }
        let mut res = Self::dp(start + 1, end, 0, memo, boxes) + (k + 1) * (k + 1);
        for i in start + 1..end {
            if boxes[i] == boxes[start] {
                res = res.max(
                    Self::dp(i, end, k + 1, memo, boxes) + Self::dp(start + 1, i, 0, memo, boxes),
                );
            }
        }
        memo.insert((start, end, k), res);
        res
    }
}

#[test]
fn test() {
    let boxes = vec![1, 3, 2, 2, 2, 3, 4, 3, 1];
    let res = 23;
    assert_eq!(Solution::remove_boxes(boxes), res);
}
