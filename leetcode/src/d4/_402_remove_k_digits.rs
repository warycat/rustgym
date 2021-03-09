struct Solution;

impl Solution {
    fn remove_kdigits(num: String, mut k: i32) -> String {
        let mut stack: Vec<char> = vec![];
        for c in num.chars() {
            while let Some(&top) = stack.last() {
                if k > 0 && top > c {
                    stack.pop();
                    k -= 1;
                } else {
                    break;
                }
            }
            stack.push(c);
        }
        while k != 0 {
            stack.pop();
            k -= 1;
        }

        let res: String = stack.into_iter().skip_while(|&c| c == '0').collect();
        if res.is_empty() {
            "0".to_string()
        } else {
            res
        }
    }
}

#[test]
fn test() {
    let num = "1432219".to_string();
    let k = 3;
    let res = "1219".to_string();
    assert_eq!(Solution::remove_kdigits(num, k), res);
    let num = "10200".to_string();
    let k = 1;
    let res = "200".to_string();
    assert_eq!(Solution::remove_kdigits(num, k), res);
    let num = "10".to_string();
    let k = 2;
    let res = "0".to_string();
    assert_eq!(Solution::remove_kdigits(num, k), res);
}
