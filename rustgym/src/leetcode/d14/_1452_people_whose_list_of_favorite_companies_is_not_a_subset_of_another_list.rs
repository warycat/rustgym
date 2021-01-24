struct Solution;
use std::collections::HashSet;

impl Solution {
    fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let people: Vec<HashSet<String>> = favorite_companies
            .into_iter()
            .map(|v| v.into_iter().collect())
            .collect();
        let n = people.len();
        let mut res = vec![];
        'outer: for i in 0..n {
            for j in 0..n {
                if i != j && people[i].is_subset(&people[j]) {
                    continue 'outer;
                }
            }
            res.push(i as i32);
        }
        res
    }
}

#[test]
fn test() {
    let favorite_companies = vec_vec_string![
        ["leetcode", "google", "facebook"],
        ["google", "microsoft"],
        ["google", "facebook"],
        ["google"],
        ["amazon"]
    ];
    let res = vec![0, 1, 4];
    assert_eq!(Solution::people_indexes(favorite_companies), res);
    let favorite_companies = vec_vec_string![
        ["leetcode", "google", "facebook"],
        ["leetcode", "amazon"],
        ["facebook", "google"]
    ];
    let res = vec![0, 1];
    assert_eq!(Solution::people_indexes(favorite_companies), res);
    let favorite_companies = vec_vec_string![["leetcode"], ["google"], ["facebook"], ["amazon"]];
    let res = vec![0, 1, 2, 3];
    assert_eq!(Solution::people_indexes(favorite_companies), res);
}
