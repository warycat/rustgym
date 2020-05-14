struct Solution;

impl Solution {
    fn min_deletion_size(a: Vec<String>) -> i32 {
        let mut d = 0;
        let a: Vec<&[u8]> = a.iter().map(|s| s.as_bytes()).collect();
        let n = a.len();
        let m = a[0].len();
        for i in 0..m {
            for j in 1..n {
                if a[j][i] < a[j - 1][i] {
                    d += 1;
                    break;
                }
            }
        }
        d
    }
}

#[test]
fn test() {
    let a: Vec<String> = vec_string!["cba", "daf", "ghi"];
    assert_eq!(Solution::min_deletion_size(a), 1);
    let a: Vec<String> = vec_string!["a", "b"];
    assert_eq!(Solution::min_deletion_size(a), 0);
    let a: Vec<String> = vec_string!["zyx", "wvu", "tsr"];
    assert_eq!(Solution::min_deletion_size(a), 3);
}
