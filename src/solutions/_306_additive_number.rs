struct Solution;

impl Solution {
    fn is_additive_number(num: String) -> bool {
        let n = num.len();
        let mut res = false;
        let mut cur: Vec<u64> = vec![];
        Self::dfs(0, &mut cur, &mut res, &num[0..n], n);
        res
    }

    fn dfs(start: usize, cur: &mut Vec<u64>, valid: &mut bool, s: &str, n: usize) {
        if start == n {
            if cur.len() >= 3 {
                *valid = true;
            }
        } else {
            for i in start + 1..=n {
                if &s[start..=start] == "0" && i - start > 1 {
                    return;
                }
                if let Ok(x) = s[start..i].parse::<u64>() {
                    let k = cur.len();
                    if k < 2 {
                        cur.push(x);
                        Self::dfs(i, cur, valid, s, n);
                        cur.pop();
                    } else {
                        if cur[k - 1] + cur[k - 2] == x {
                            cur.push(x);
                            Self::dfs(i, cur, valid, s, n);
                            cur.pop();
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test() {
    let num = "112358".to_string();
    let res = true;
    assert_eq!(Solution::is_additive_number(num), res);
    let num = "199100199".to_string();
    let res = true;
    assert_eq!(Solution::is_additive_number(num), res);
    let num = "101".to_string();
    let res = true;
    assert_eq!(Solution::is_additive_number(num), res);
    let num = "12012122436".to_string();
    let res = true;
    assert_eq!(Solution::is_additive_number(num), res);
    let num = "121474836472147483648".to_string();
    let res = true;
    assert_eq!(Solution::is_additive_number(num), res);
}
