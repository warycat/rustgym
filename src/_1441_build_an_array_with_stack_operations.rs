struct Solution;

impl Solution {
    fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut res = vec![];
        let m = target.len();
        let mut j = 0;
        for i in 1..=n {
            if j == m {
                break;
            }
            res.push("Push".to_string());
            if target[j] == i {
                j += 1;
            } else {
                res.push("Pop".to_string());
            }
        }
        res
    }
}

#[test]
fn test() {
    let target = vec![1, 3];
    let n = 3;
    let res = vec_string!["Push", "Push", "Pop", "Push"];
    assert_eq!(Solution::build_array(target, n), res);
    let target = vec![1, 2, 3];
    let n = 3;
    let res = vec_string!["Push", "Push", "Push"];
    assert_eq!(Solution::build_array(target, n), res);
    let target = vec![1, 2];
    let n = 4;
    let res = vec_string!["Push", "Push"];
    assert_eq!(Solution::build_array(target, n), res);
    let target = vec![2, 3, 4];
    let n = 4;
    let res = vec_string!["Push", "Pop", "Push", "Push", "Push"];
    assert_eq!(Solution::build_array(target, n), res);
}
