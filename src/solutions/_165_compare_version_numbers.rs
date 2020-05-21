struct Solution;

impl Solution {
    fn compare_version(version1: String, version2: String) -> i32 {
        let version1: Vec<i32> = version1
            .split_terminator('.')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let version2: Vec<i32> = version2
            .split_terminator('.')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let n1 = version1.len();
        let n2 = version2.len();
        for i in 0..n1.max(n2) {
            let x1 = version1.get(i).unwrap_or(&0);
            let x2 = version2.get(i).unwrap_or(&0);
            if x1 > x2 {
                return 1;
            }
            if x1 < x2 {
                return -1;
            }
        }
        0
    }
}

#[test]
fn test() {
    let version1 = "0.1".to_string();
    let version2 = "1.1".to_string();
    let res = -1;
    assert_eq!(Solution::compare_version(version1, version2), res);
    let version1 = "1.0.1".to_string();
    let version2 = "1".to_string();
    let res = 1;
    assert_eq!(Solution::compare_version(version1, version2), res);
    let version1 = "7.5.2.4".to_string();
    let version2 = "7.5.3".to_string();
    let res = -1;
    assert_eq!(Solution::compare_version(version1, version2), res);
    let version1 = "1.01".to_string();
    let version2 = "1.001".to_string();
    let res = 0;
    assert_eq!(Solution::compare_version(version1, version2), res);
    let version1 = "1.0".to_string();
    let version2 = "1.0.0".to_string();
    let res = 0;
    assert_eq!(Solution::compare_version(version1, version2), res);
}
