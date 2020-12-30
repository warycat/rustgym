struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        let n = heights.len();
        let mut replacement: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for i in 1..n {
            if heights[i - 1] < heights[i] {
                replacement.push(Reverse(heights[i] - heights[i - 1]));
                if ladders > 0 {
                    ladders -= 1;
                } else {
                    let min = replacement.pop().unwrap().0;
                    if min <= bricks {
                        bricks -= min;
                    } else {
                        return (i - 1) as i32;
                    }
                }
            }
        }
        (n - 1) as i32
    }
}

#[test]
fn test() {
    let heights = vec![4, 2, 7, 6, 9, 14, 12];
    let bricks = 5;
    let ladders = 1;
    let res = 4;
    assert_eq!(Solution::furthest_building(heights, bricks, ladders), res);
    let heights = vec![4, 12, 2, 7, 3, 18, 20, 3, 19];
    let bricks = 10;
    let ladders = 2;
    let res = 7;
    assert_eq!(Solution::furthest_building(heights, bricks, ladders), res);
    let heights = vec![14, 3, 19, 3];
    let bricks = 17;
    let ladders = 0;
    let res = 3;
    assert_eq!(Solution::furthest_building(heights, bricks, ladders), res);
}
