struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn maximum_number_of_ones(width: i32, height: i32, side_length: i32, max_ones: i32) -> i32 {
        let width = width as usize;
        let height = height as usize;
        let side_length = side_length as usize;
        let max_ones = max_ones as usize;
        let mut queue: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
        for i in 0..side_length {
            for j in 0..side_length {
                let rows = 1 + (height - (i + 1)) / side_length;
                let cols = 1 + (width - (j + 1)) / side_length;
                queue.push(Reverse(rows * cols));
                if queue.len() > max_ones {
                    queue.pop();
                }
            }
        }
        queue.into_iter().map(|t| t.0).sum::<usize>() as i32
    }
}

#[test]
fn test() {
    let width = 3;
    let height = 3;
    let side_length = 2;
    let max_ones = 1;
    let res = 4;
    assert_eq!(
        Solution::maximum_number_of_ones(width, height, side_length, max_ones),
        res
    );
    let width = 3;
    let height = 3;
    let side_length = 2;
    let max_ones = 2;
    let res = 6;
    assert_eq!(
        Solution::maximum_number_of_ones(width, height, side_length, max_ones),
        res
    );
}
