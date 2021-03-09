struct Solution;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let n = transactions.len();
        let mut hm: HashMap<String, Vec<(i32, String, String)>> = HashMap::new();
        let mut hs: HashSet<String> = HashSet::new();
        for i in 0..n {
            let transaction = &transactions[i];
            let mut v: Vec<String> = transaction
                .split_terminator(',')
                .map(|s| s.to_string())
                .collect();
            let city = v.pop().unwrap();
            let amount = v.pop().unwrap().parse::<i32>().unwrap();
            let time = v.pop().unwrap().parse::<i32>().unwrap();
            let name = v.pop().unwrap();
            hm.entry(name)
                .or_default()
                .push((time, city, transaction.clone()));
            if amount > 1000 {
                hs.insert(transactions[i].clone());
            }
        }
        for transactions in hm.values() {
            let n = transactions.len();
            for i in 0..n {
                for j in i + 1..n {
                    let ti = &transactions[i];
                    let tj = &transactions[j];
                    if (ti.0 - tj.0).abs() <= 60 && ti.1 != tj.1 {
                        hs.insert(ti.2.clone());
                        hs.insert(tj.2.clone());
                    }
                }
            }
        }
        hs.drain().collect()
    }
}

#[test]
fn test() {
    let transactions = vec_string!["alice,20,800,mtv", "alice,50,100,beijing"];
    let mut res = vec_string!["alice,20,800,mtv", "alice,50,100,beijing"];
    let mut ans = Solution::invalid_transactions(transactions);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let transactions = vec_string!["alice,20,800,mtv", "alice,50,1200,mtv"];
    let mut res = vec_string!["alice,50,1200,mtv"];
    let mut ans = Solution::invalid_transactions(transactions);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let transactions = vec_string!["alice,20,800,mtv", "bob,50,1200,mtv"];
    let mut res = vec_string!["bob,50,1200,mtv"];
    let mut ans = Solution::invalid_transactions(transactions);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
