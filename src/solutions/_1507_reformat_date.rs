struct Solution;
use std::collections::HashMap;

impl Solution {
    fn reformat_date(date: String) -> String {
        let month: HashMap<String, i32> = vec![
            ("Jan", 1),
            ("Feb", 2),
            ("Mar", 3),
            ("Apr", 4),
            ("May", 5),
            ("Jun", 6),
            ("Jul", 7),
            ("Aug", 8),
            ("Sep", 9),
            ("Oct", 10),
            ("Nov", 11),
            ("Dec", 12),
        ]
        .into_iter()
        .map(|(m, mm)| (m.to_string(), mm))
        .collect();
        let mut v: Vec<String> = date.split_whitespace().map(|s| s.to_string()).collect();
        let yyyy = v.pop().unwrap();
        let mm = month[&v.pop().unwrap()];
        let mut dd = v.pop().unwrap();
        dd.pop();
        dd.pop();
        let dd = dd.parse::<i32>().unwrap();
        format!("{}-{:02}-{:02}", yyyy, mm, dd)
    }
}

#[test]
fn test() {
    let date = "20th Oct 2052".to_string();
    let res = "2052-10-20".to_string();
    assert_eq!(Solution::reformat_date(date), res);
    let date = "6th Jun 1933".to_string();
    let res = "1933-06-06".to_string();
    assert_eq!(Solution::reformat_date(date), res);
    let date = "26th May 1960".to_string();
    let res = "1960-05-26".to_string();
    assert_eq!(Solution::reformat_date(date), res);
}
