struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn num_of_ways(n: i32) -> i32 {
        let n = n as usize;
        let mut a121: i64 = 6;
        let mut a123: i64 = 6;
        for _ in 1..n {
            let b121 = a121 * 3 + a123 * 2;
            let b123 = a121 * 2 + a123 * 2;
            a121 = b121 % MOD;
            a123 = b123 % MOD;
        }
        ((a121 + a123) % MOD) as i32
    }
}

#[test]
fn test() {
    let n = 1;
    let res = 12;
    assert_eq!(Solution::num_of_ways(n), res);
    let n = 2;
    let res = 54;
    assert_eq!(Solution::num_of_ways(n), res);
    let n = 7;
    let res = 106494;
    assert_eq!(Solution::num_of_ways(n), res);
    let n = 5000;
    let res = 30228214;
    assert_eq!(Solution::num_of_ways(n), res);
}
