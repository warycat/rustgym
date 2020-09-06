struct Solution;

impl Solution {
    fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        let mut stack: Vec<(i32, i32)> = vec![(0, 0)];
        heights.push(0);
        let n = heights.len();
        let mut res = 0;
        for i in 0..n {
            let x1 = (i + 1) as i32;
            let y1 = heights[i];
            let mut x3 = x1;
            while let Some(&(x2, y2)) = stack.last() {
                if y2 > y1 {
                    stack.pop();
                    res = res.max((x1 - x2) * y2);
                    x3 = x2;
                } else {
                    stack.push((x3, y1));
                    break;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let heights = vec![2, 1, 5, 6, 2, 3];
    let res = 10;
    assert_eq!(Solution::largest_rectangle_area(heights), res);
    let heights = vec![2, 1, 2];
    let res = 3;
    assert_eq!(Solution::largest_rectangle_area(heights), res);
}
