struct Solution;

impl Solution {
    fn min_operations(logs: Vec<String>) -> i32 {
        let mut res = 0;
        for log in logs {
            if log == "../" {
                if res > 0 {
                    res -= 1;
                }
            } else if log != "./" {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let logs = vec_string!["d1/", "d2/", "../", "d21/", "./"];
    let res = 2;
    assert_eq!(Solution::min_operations(logs), res);
    let logs = vec_string!["d1/", "d2/", "./", "d3/", "../", "d31/"];
    let res = 3;
    assert_eq!(Solution::min_operations(logs), res);
    let logs = vec_string!["d1/", "../", "../", "../"];
    let res = 0;
    assert_eq!(Solution::min_operations(logs), res);
}
