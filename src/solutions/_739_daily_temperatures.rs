struct Solution;

impl Solution {
    fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut stack: Vec<usize> = vec![];
        let n = t.len();
        for i in (0..n).rev() {
            while let Some(j) = stack.pop() {
                if t[j] > t[i] {
                    stack.push(j);
                    break;
                }
            }
            if let Some(j) = stack.last() {
                res.push((j - i) as i32);
            } else {
                res.push(0);
            }
            stack.push(i)
        }
        res.into_iter().rev().collect()
    }
}

#[test]
fn test() {
    let t = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let res = vec![1, 1, 4, 2, 1, 1, 0, 0];
    assert_eq!(Solution::daily_temperatures(t), res);
}
