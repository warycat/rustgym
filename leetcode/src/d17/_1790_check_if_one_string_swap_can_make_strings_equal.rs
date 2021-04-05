struct Solution;

impl Solution {
    fn are_almost_equal(s1: String, s2: String) -> bool {
        let n1 = s1.len();
        let n2 = s2.len();
        if n1 != n2 {
            return false;
        }
        let arr: Vec<(char, char)> = s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).collect();
        arr.is_empty() || (arr.len() == 2 && arr[0].0 == arr[1].1 && arr[0].1 == arr[1].0)
    }
}

#[test]
fn test() {
    let s1 = "bank".to_string();
    let s2 = "kanb".to_string();
    let res = true;
    assert_eq!(Solution::are_almost_equal(s1, s2), res);
    let s1 = "attack".to_string();
    let s2 = "defend".to_string();
    let res = false;
    assert_eq!(Solution::are_almost_equal(s1, s2), res);
    let s1 = "kelb".to_string();
    let s2 = "kelb".to_string();
    let res = true;
    assert_eq!(Solution::are_almost_equal(s1, s2), res);
    let s1 = "abcd".to_string();
    let s2 = "dcba".to_string();
    let res = false;
    assert_eq!(Solution::are_almost_equal(s1, s2), res);
    let s1 = "caa".to_string();
    let s2 = "aaz".to_string();
    let res = false;
    assert_eq!(Solution::are_almost_equal(s1, s2), res);
}
