struct Solution;

impl Solution {
    fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut first: Option<usize> = None;
        let mut last: Option<usize> = None;
        let mut prev: Option<usize> = None;
        let n = seats.len();
        let mut max = 0;
        for i in 0..n {
            if seats[i] == 1 {
                if first.is_none() {
                    first = Some(i);
                }
                if let Some(j) = prev {
                    max = usize::max((i - j) / 2, max);
                }
                prev = Some(i);
                last = Some(i);
            }
        }
        max = usize::max(first.unwrap(), max);
        max = usize::max(n - 1 - last.unwrap(), max);
        max as i32
    }
}

#[test]
fn test() {
    let seats = vec![1, 0, 0, 0, 1, 0, 1];
    assert_eq!(Solution::max_dist_to_closest(seats), 2);
    let seats = vec![1, 0, 0, 0];
    assert_eq!(Solution::max_dist_to_closest(seats), 3);
}
