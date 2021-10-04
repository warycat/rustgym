struct Solution;

impl Solution {
    fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut res = 0;
        for op in operations {
            if op.contains('+') {
                res += 1;
            } else {
                res -= 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let operations = vec_string!["--X", "X++", "X++"];
    let res = 1;
    assert_eq!(Solution::final_value_after_operations(operations), res);
    let operations = vec_string!["++X", "++X", "X++"];
    let res = 3;
    assert_eq!(Solution::final_value_after_operations(operations), res);
    let operations = vec_string!["X++", "++X", "--X", "X--"];
    let res = 0;
    assert_eq!(Solution::final_value_after_operations(operations), res);
}
