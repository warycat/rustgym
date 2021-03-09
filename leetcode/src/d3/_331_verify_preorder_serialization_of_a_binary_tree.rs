struct Solution;

impl Solution {
    fn is_valid_serialization(preorder: String) -> bool {
        let mut stack: Vec<char> = vec![];
        for tok in preorder.split(',') {
            if tok == "#" {
                while let Some('#') = stack.last() {
                    stack.pop();
                    if stack.pop().is_none() {
                        return false;
                    };
                }
                stack.push('#');
            } else {
                stack.push('$');
            }
        }
        stack == vec!['#']
    }
}

#[test]
fn test() {
    let preorder = "9,3,4,#,#,1,#,#,2,#,6,#,#".to_string();
    let res = true;
    assert_eq!(Solution::is_valid_serialization(preorder), res);
    let preorder = "1,#".to_string();
    let res = false;
    assert_eq!(Solution::is_valid_serialization(preorder), res);
    let preorder = "9,#,#,1".to_string();
    let res = false;
    assert_eq!(Solution::is_valid_serialization(preorder), res);
    let preorder = "1,#,#,#,#".to_string();
    let res = false;
    assert_eq!(Solution::is_valid_serialization(preorder), res);
}
