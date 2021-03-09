struct Solution;

impl Solution {
    fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let mut cur: String = "".to_string();
        Self::dfs(n, n, &mut cur, &mut res);
        res
    }
    fn dfs(left: i32, right: i32, cur: &mut String, all: &mut Vec<String>) {
        if left == 0 && right == 0 {
            all.push(cur.to_string());
        } else {
            if left > 0 {
                cur.push('(');
                Self::dfs(left - 1, right, cur, all);
                cur.pop();
            }
            if right > left {
                cur.push(')');
                Self::dfs(left, right - 1, cur, all);
                cur.pop();
            }
        }
    }
}

#[test]
fn test() {
    let res: Vec<String> = vec_string!["((()))", "(()())", "(())()", "()(())", "()()()"];
    assert_eq!(Solution::generate_parenthesis(3), res);
}
