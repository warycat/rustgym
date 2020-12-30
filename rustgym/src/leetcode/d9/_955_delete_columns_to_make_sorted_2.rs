struct Solution;

impl Solution {
    fn min_deletion_size(a: Vec<String>) -> i32 {
        let n = a.len();
        let mut sorted = vec!["".to_string(); n];
        let a: Vec<Vec<char>> = a.into_iter().map(|s| s.chars().collect()).collect();
        let m = a[0].len();
        for j in 0..m {
            for i in 0..n {
                sorted[i].push(a[i][j]);
            }
            if sorted.windows(2).any(|w| w[0] > w[1]) {
                for i in 0..n {
                    sorted[i].pop();
                }
            }
        }
        (m - sorted[0].len()) as i32
    }
}

#[test]
fn test() {
    let a = vec_string!["ca", "bb", "ac"];
    let res = 1;
    assert_eq!(Solution::min_deletion_size(a), res);
    let a = vec_string!["xc", "yb", "za"];
    let res = 0;
    assert_eq!(Solution::min_deletion_size(a), res);
    let a = vec_string!["zyx", "wvu", "tsr"];
    let res = 3;
    assert_eq!(Solution::min_deletion_size(a), res);
}
