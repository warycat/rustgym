struct Solution;

impl Solution {
    fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut in_block = false;
        let mut line = "".to_string();
        let mut res = vec![];
        for s in source {
            let mut it = s.chars().peekable();
            while let Some(c) = it.next() {
                if in_block {
                    if c == '*' {
                        if let Some(&'/') = it.peek() {
                            it.next();
                            in_block = false;
                        }
                    }
                } else {
                    if c == '/' {
                        match it.peek() {
                            Some(&'/') => {
                                break;
                            }
                            Some(&'*') => {
                                it.next();
                                in_block = true;
                                continue;
                            }
                            _ => {}
                        }
                    }
                    line.push(c);
                }
            }
            if !in_block && !line.is_empty() {
                res.push(line);
                line = "".to_string();
            }
        }
        res
    }
}

#[test]
fn test() {
    let source = vec_string![
        "/*Test program */",
        "int main()",
        "{ ",
        "  // variable declaration ",
        "int a, b, c;",
        "/* This is a test",
        "   multiline  ",
        "   comment for ",
        "   testing */",
        "a = b + c;",
        "}"
    ];
    let res = vec_string!["int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"];
    assert_eq!(Solution::remove_comments(source), res);
}
