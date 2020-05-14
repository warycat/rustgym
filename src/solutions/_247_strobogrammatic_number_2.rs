struct Solution;
use std::collections::HashMap;

impl Solution {
    fn find_strobogrammatic(n: i32) -> Vec<String> {
        let mut res = vec![];
        let hm: HashMap<char, char> =
            vec![('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')]
                .into_iter()
                .collect();
        let n = n as usize;
        let mut cur: Vec<char> = vec![];
        Self::dfs(0, &mut cur, &mut res, &hm, n);
        res
    }

    fn dfs(
        i: usize,
        cur: &mut Vec<char>,
        all: &mut Vec<String>,
        hm: &HashMap<char, char>,
        n: usize,
    ) {
        if i == n {
            all.push(cur.iter().collect());
        } else {
            for (&k, &v) in hm {
                if n != 1 && i == 0 && k == '0' {
                    continue;
                }
                let j = n - 1 - i;
                if j < i && cur[j] != v {
                    continue;
                }
                if j == i && k != v {
                    continue;
                }
                cur.push(k);
                Self::dfs(i + 1, cur, all, hm, n);
                cur.pop();
            }
        }
    }
}

#[test]
fn test() {
    let n = 1;
    let mut res = vec_string!["0", "1", "8"];
    let mut ans = Solution::find_strobogrammatic(n);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let n = 2;
    let mut res = vec_string!["11", "69", "88", "96"];
    let mut ans = Solution::find_strobogrammatic(n);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
