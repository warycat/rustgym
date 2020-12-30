struct Solution;
use std::collections::HashSet;

impl Solution {
    fn simplified_fractions(n: i32) -> Vec<String> {
        let mut hs: HashSet<String> = HashSet::new();
        for i in 2..=n {
            for j in 1..i {
                let k = Self::gcd(i, j);
                hs.insert(format!("{}/{}", j / k, i / k));
            }
        }
        hs.into_iter().collect()
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while a != 0 {
            let t = a;
            a = b % t;
            b = t;
        }
        b
    }
}

#[test]
fn test() {
    let n = 2;
    let res = vec_string!["1/2"];
    assert_eq!(Solution::simplified_fractions(n), res);
    let n = 3;
    let mut res = vec_string!["1/2", "1/3", "2/3"];
    let mut ans = Solution::simplified_fractions(n);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let n = 4;
    let mut res = vec_string!["1/2", "1/3", "1/4", "2/3", "3/4"];
    let mut ans = Solution::simplified_fractions(n);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
