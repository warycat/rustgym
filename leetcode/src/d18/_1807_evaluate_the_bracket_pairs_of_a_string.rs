struct Solution;

use std::collections::HashMap;

impl Solution {
    fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let mut hm: HashMap<String, String> = HashMap::new();
        for pair in knowledge {
            hm.insert(pair[0].to_string(), pair[1].to_string());
        }
        let mut res = "".to_string();
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut bracket = false;
        let mut temp = vec![];
        for i in 0..n {
            match s[i] {
                '(' => {
                    bracket = true;
                }
                ')' => {
                    bracket = false;
                    let key: String = temp.iter().collect();
                    if let Some(value) = hm.get(&key) {
                        res.push_str(value);
                    } else {
                        res.push('?');
                    }
                    temp = vec![];
                }
                _ => {
                    if bracket {
                        temp.push(s[i]);
                    } else {
                        res.push(s[i])
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "(name)is(age)yearsold".to_string();
    let knowledge = vec_vec_string![["name", "bob"], ["age", "two"]];
    let res = "bobistwoyearsold".to_string();
    assert_eq!(Solution::evaluate(s, knowledge), res);
    let s = "hi(name)".to_string();
    let knowledge = vec_vec_string![["a", "b"]];
    let res = "hi?".to_string();
    assert_eq!(Solution::evaluate(s, knowledge), res);
    let s = "(a)(a)(a)aaa".to_string();
    let knowledge = vec_vec_string![["a", "yes"]];
    let res = "yesyesyesaaa".to_string();
    assert_eq!(Solution::evaluate(s, knowledge), res);
    let s = "(a)(b)".to_string();
    let knowledge = vec_vec_string![["a", "b"], ["b", "a"]];
    let res = "ba".to_string();
    assert_eq!(Solution::evaluate(s, knowledge), res);
}
