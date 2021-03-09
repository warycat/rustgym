struct Solution;

impl Solution {
    fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::from("");
        }
        let ss: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();
        let n = ss.iter().map(|s| s.len()).min().unwrap();
        let mut prefix: Vec<char> = vec![];
        for i in 0..n {
            let c = ss[0][i];
            if ss.iter().all(|s| s[i] == c) {
                prefix.push(c);
            } else {
                break;
            }
        }
        prefix.iter().collect()
    }
}

#[test]
fn test() {
    let ss: Vec<String> = vec_string!["flower", "flow", "flight"];
    assert_eq!(Solution::longest_common_prefix(ss), String::from("fl"));
}
