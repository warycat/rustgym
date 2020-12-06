struct Solution;

impl Solution {
    fn interpret(command: String) -> String {
        let mut it = command.chars().peekable();
        let mut res = "".to_string();
        while let Some(c) = it.next() {
            match c {
                'G' => {
                    res.push('G');
                }
                _ => {
                    let next = it.next().unwrap();
                    if next == ')' {
                        res.push('o');
                    } else {
                        it.next();
                        it.next();
                        res.push('a');
                        res.push('l');
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let command = "G()(al)".to_string();
    let res = "Goal".to_string();
    assert_eq!(Solution::interpret(command), res);
    let command = "G()()()()(al)".to_string();
    let res = "Gooooal".to_string();
    assert_eq!(Solution::interpret(command), res);
    let command = "(al)G(al)()()G".to_string();
    let res = "alGalooG".to_string();
    assert_eq!(Solution::interpret(command), res);
}
