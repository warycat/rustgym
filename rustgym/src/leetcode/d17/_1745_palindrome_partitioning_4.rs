struct Solution;

impl Solution {
    fn check_partitioning(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut palindrome: Vec<Vec<bool>> = vec![vec![false; 2000]; 2000];
        for size in 1..=n {
            for i in 0..=n - size {
                let j = i + size - 1;
                if s[i] == s[j] {
                    if i == j || i + 1 == j {
                        palindrome[i][j] = true;
                    } else {
                        palindrome[i][j] = palindrome[i + 1][j - 1];
                    }
                }
            }
        }
        for i in 1..n {
            for j in i..n - 1 {
                if palindrome[0][i - 1] && palindrome[i][j] && palindrome[j + 1][n - 1] {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let s = "abcbdd".to_string();
    let res = true;
    assert_eq!(Solution::check_partitioning(s), res);
    let s = "bcbddxy".to_string();
    let res = false;
    assert_eq!(Solution::check_partitioning(s), res);
    let s = "aaa".to_string();
    let res = true;
    assert_eq!(Solution::check_partitioning(s), res);
}
