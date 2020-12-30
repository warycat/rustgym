struct Solution;

impl Solution {
    fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = vec![];
        let mut res = "".to_string();
        for s in path.split_terminator('/') {
            match s {
                ".." => {
                    stack.pop();
                }
                "" | "." => {
                    continue;
                }
                _ => {
                    stack.push(s);
                }
            }
        }
        for s in stack {
            res += "/";
            res += s;
        }
        if res.is_empty() {
            res += "/";
        }
        res
    }
}

#[test]
fn test() {
    let path = "/home/".to_string();
    let res = "/home".to_string();
    assert_eq!(Solution::simplify_path(path), res);
    let path = "/../".to_string();
    let res = "/".to_string();
    assert_eq!(Solution::simplify_path(path), res);
    let path = "/home//foo/".to_string();
    let res = "/home/foo".to_string();
    assert_eq!(Solution::simplify_path(path), res);
    let path = "/a/./b/../../c/".to_string();
    let res = "/c".to_string();
    assert_eq!(Solution::simplify_path(path), res);
    let path = "/a/../../b/../c//.//".to_string();
    let res = "/c".to_string();
    assert_eq!(Solution::simplify_path(path), res);
    let path = "/a//b////c/d//././/..".to_string();
    let res = "/a/b/c".to_string();
    assert_eq!(Solution::simplify_path(path), res);
}
