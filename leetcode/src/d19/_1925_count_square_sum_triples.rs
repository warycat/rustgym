struct Solution;

impl Solution {
    fn count_triples(n: i32) -> i32 {
        let mut res = 0;
        for a in 1..n {
            for b in a + 1..n {
                for c in b + 1..=n {
                    if a * a + b * b == c * c {
                        res += 1;
                    }
                }
            }
        }
        2 * res
    }
}

#[test]
fn test() {
    let n = 5;
    let res = 2;
    assert_eq!(Solution::count_triples(n), res);
    let n = 10;
    let res = 4;
    assert_eq!(Solution::count_triples(n), res);
}
