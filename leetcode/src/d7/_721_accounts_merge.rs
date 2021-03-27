struct Solution;

use rustgym_util::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;

impl Solution {
    fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let n = accounts.len();
        let mut btm: BTreeMap<&str, &str> = BTreeMap::new();
        for i in 0..n {
            let m = accounts[i].len();
            let name = &accounts[i][0];
            for j in 1..m {
                let email = &accounts[i][j];
                btm.insert(email, name);
            }
        }
        let count = btm.len();
        let mut uf = UnionFind::new(count);
        let mut owners = vec![];
        let mut emails = vec![];
        let mut ids = HashMap::new();
        for (i, (email, name)) in btm.into_iter().enumerate() {
            emails.push(email.to_string());
            owners.push(name.to_string());
            ids.insert(email.to_string(), i);
        }

        for i in 0..n {
            let m = accounts[i].len();
            for j in 2..m {
                let email_a = &accounts[i][j - 1];
                let email_b = &accounts[i][j];
                let id_a = ids[email_a];
                let id_b = ids[email_b];
                uf.union(id_a, id_b);
            }
        }
        let mut res: Vec<Vec<String>> = vec![];
        let mut hm: HashMap<usize, BTreeSet<usize>> = HashMap::new();
        for i in 0..count {
            let group_id = uf.find(i);
            hm.entry(group_id).or_default().insert(i);
        }
        for (group_id, ids) in hm.into_iter() {
            let mut v: Vec<String> = vec![owners[group_id].clone()];
            for id in ids {
                v.push(emails[id].to_string());
            }
            res.push(v);
        }
        res
    }
}

#[test]
fn test() {
    let accounts = vec![
        vec_string!["John", "johnsmith@mail.com", "john00@mail.com"],
        vec_string!["John", "johnnybravo@mail.com"],
        vec_string!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
        vec_string!["Mary", "mary@mail.com"],
    ];
    let mut ans = vec![
        vec_string![
            "John",
            "john00@mail.com",
            "john_newyork@mail.com",
            "johnsmith@mail.com"
        ],
        vec_string!["John", "johnnybravo@mail.com"],
        vec_string!["Mary", "mary@mail.com"],
    ];
    let mut res = Solution::accounts_merge(accounts);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(ans, res);
}
