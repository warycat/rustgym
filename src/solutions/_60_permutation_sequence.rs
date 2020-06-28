struct Solution;

impl Solution {
    fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut k = k as usize - 1;
        let mut nums: Vec<usize> = (1..=n).collect();
        let mut factorial = vec![1];
        let mut prev = 1;
        for x in 1..n {
            prev *= x;
            factorial.push(prev);
        }
        let mut res = vec![];
        for i in 0..n {
            let index = k / factorial[n - 1 - i];
            res.push(nums.remove(index));
            k %= factorial[n - 1 - i];
        }
        res.into_iter().map(|x| (x as u8 + b'0') as char).collect()
    }
}

#[test]
fn test() {
    let n = 3;
    let k = 3;
    let res = "213".to_string();
    assert_eq!(Solution::get_permutation(n, k), res);
    let n = 4;
    let k = 9;
    let res = "2314".to_string();
    assert_eq!(Solution::get_permutation(n, k), res);
    let n = 3;
    let k = 5;
    let res = "312".to_string();
    assert_eq!(Solution::get_permutation(n, k), res);
}
