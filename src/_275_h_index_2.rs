struct Solution;
use std::cmp::Ordering::*;

impl Solution {
    fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut l = 0;
        let mut r = n;
        while l < r {
            let m = l + (r - l) / 2;
            match (citations[m] as usize).cmp(&(n - m)) {
                Equal => {
                    return (n - m) as i32;
                }
                Less => {
                    l = m + 1;
                }
                Greater => {
                    r = m;
                }
            }
        }
        (n - l) as i32
    }
}

#[test]
fn test() {
    let citations = vec![0, 1, 3, 5, 6];
    let res = 3;
    assert_eq!(Solution::h_index(citations), res);
    let citations = vec![1, 2];
    let res = 1;
    assert_eq!(Solution::h_index(citations), res);
    let citations = vec![11, 15];
    let res = 2;
    assert_eq!(Solution::h_index(citations), res);
}
