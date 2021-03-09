struct Solution;

impl Solution {
    fn get_happy_string(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize;
        let mut all: Vec<String> = vec![];
        let mut cur: Vec<u8> = vec![];
        Self::dfs(0, &mut cur, &mut all, n, k);
        if all.len() < k {
            "".to_string()
        } else {
            all.pop().unwrap()
        }
    }
    fn dfs(start: usize, cur: &mut Vec<u8>, all: &mut Vec<String>, n: usize, k: usize) {
        if all.len() == k {
            return;
        }
        if start == n {
            let s: String = cur.iter().map(|&i| (b'a' + i) as char).collect();
            all.push(s);
        } else {
            for i in 0..3 {
                if start > 0 && i == cur[start - 1] {
                    continue;
                }
                cur.push(i);
                Self::dfs(start + 1, cur, all, n, k);
                cur.pop();
            }
        }
    }
}

#[test]
fn test() {
    let n = 1;
    let k = 3;
    let res = "c".to_string();
    assert_eq!(Solution::get_happy_string(n, k), res);
    let n = 1;
    let k = 4;
    let res = "".to_string();
    assert_eq!(Solution::get_happy_string(n, k), res);
    let n = 3;
    let k = 9;
    let res = "cab".to_string();
    assert_eq!(Solution::get_happy_string(n, k), res);
    let n = 2;
    let k = 7;
    let res = "".to_string();
    assert_eq!(Solution::get_happy_string(n, k), res);
    let n = 10;
    let k = 100;
    let res = "abacbabacb".to_string();
    assert_eq!(Solution::get_happy_string(n, k), res);
}
