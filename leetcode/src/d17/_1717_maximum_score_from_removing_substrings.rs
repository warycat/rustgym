struct Solution;

impl Solution {
    fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let (a, b, x, y) = if x >= y {
            ('a', 'b', x, y)
        } else {
            ('b', 'a', y, x)
        };
        let mut res = 0;
        let mut stack1 = vec![];
        for c in s.chars() {
            if c == b {
                if let Some(&d) = stack1.last() {
                    if d == a {
                        res += x;
                        stack1.pop().unwrap();
                        continue;
                    }
                }
            }
            stack1.push(c);
        }
        let mut stack2 = vec![];
        for c in stack1 {
            if c == a {
                if let Some(&d) = stack2.last() {
                    if d == b {
                        res += y;
                        stack2.pop().unwrap();
                        continue;
                    }
                }
            }
            stack2.push(c);
        }
        res
    }
}

#[test]
fn test() {
    let s = "cdbcbbaaabab".to_string();
    let x = 4;
    let y = 5;
    let res = 19;
    assert_eq!(Solution::maximum_gain(s, x, y), res);
    let s = "aabbaaxybbaabb".to_string();
    let x = 5;
    let y = 4;
    let res = 20;
    assert_eq!(Solution::maximum_gain(s, x, y), res);
}
