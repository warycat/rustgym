struct Solution;

impl Solution {
    fn gen(res: &mut Vec<String>, v: &mut Vec<char>, left: i32, right: i32) {
        if left == 0 && right == 0 {
            res.push(v.iter().collect::<String>());
        } else {
            if left > 0 {
                v.push('(');
                Self::gen(res, v, left - 1, right);
                v.pop();
            }
            if right > left {
                v.push(')');
                Self::gen(res, v, left, right - 1);
                v.pop();
            }
        }
    }
    fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let mut v: Vec<char> = vec![];
        Self::gen(&mut res, &mut v, n, n);
        res
    }
}

#[test]
fn test() {
    let res: Vec<String> = vec_string!["((()))", "(()())", "(())()", "()(())", "()()()"];
    assert_eq!(Solution::generate_parenthesis(3), res);
}
