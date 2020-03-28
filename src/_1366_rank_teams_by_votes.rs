struct Solution;
use std::cmp::Ordering::*;

impl Solution {
    fn rank_teams(votes: Vec<String>) -> String {
        let n = votes[0].len();
        let mut count: Vec<Vec<usize>> = vec![vec![0; n]; 26];
        for s in &votes {
            for (i, c) in s.bytes().enumerate() {
                count[(c - b'A') as usize][i] += 1;
            }
        }
        let mut v: Vec<u8> = votes[0].bytes().map(|b| b - b'A').collect();
        v.sort_by(|&a, &b| {
            for i in 0..n {
                match &count[a as usize][i].cmp(&count[b as usize][i]) {
                    Equal => {}
                    Less => {
                        return Greater;
                    }
                    Greater => {
                        return Less;
                    }
                }
            }
            a.cmp(&b)
        });
        v.into_iter().map(|b| (b'A' + b) as char).collect()
    }
}

#[test]
fn test() {
    let votes = vec_string!["ABC", "ACB", "ABC", "ACB", "ACB"];
    let res = "ACB".to_string();
    assert_eq!(Solution::rank_teams(votes), res);
    let votes = vec_string!["WXYZ", "XYZW"];
    let res = "XWYZ".to_string();
    assert_eq!(Solution::rank_teams(votes), res);
    let votes = vec_string!["ZMNAGUEDSJYLBOPHRQICWFXTVK"];
    let res = "ZMNAGUEDSJYLBOPHRQICWFXTVK".to_string();
    assert_eq!(Solution::rank_teams(votes), res);
    let votes = vec_string!["BCA", "CAB", "CBA", "ABC", "ACB", "BAC"];
    let res = "ABC".to_string();
    assert_eq!(Solution::rank_teams(votes), res);
    let votes = vec_string!["M", "M", "M", "M"];
    let res = "M".to_string();
    assert_eq!(Solution::rank_teams(votes), res);
}
