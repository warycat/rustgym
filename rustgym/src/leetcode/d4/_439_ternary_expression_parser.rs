struct Solution;

impl Solution {
    fn parse_ternary(expression: String) -> String {
        let expression: Vec<char> = expression.chars().collect();
        Self::parse(&expression).iter().copied().collect()
    }

    fn parse(expression: &[char]) -> &[char] {
        let n = expression.len();
        if n == 1 {
            expression
        } else {
            let mut count = 1;
            for i in 2..n {
                if expression[i] == '?' {
                    count += 1;
                    continue;
                }
                if expression[i] == ':' {
                    count -= 1;
                    if count == 0 {
                        return if expression[0] == 'T' {
                            Self::parse(&expression[2..i])
                        } else {
                            Self::parse(&expression[i + 1..])
                        };
                    }
                }
            }
            &[]
        }
    }
}

#[test]
fn test() {
    let expression = "T?2:3".to_string();
    let res = "2".to_string();
    assert_eq!(Solution::parse_ternary(expression), res);
    let expression = "F?1:T?4:5".to_string();
    let res = "4".to_string();
    assert_eq!(Solution::parse_ternary(expression), res);
    let expression = "T?T?F:5:3".to_string();
    let res = "F".to_string();
    assert_eq!(Solution::parse_ternary(expression), res);
}
