struct Solution;

impl Solution {
    fn count_vowel_strings(n: i32) -> i32 {
        let n = n as usize;
        let mut res = 0;
        for i in 0..5 {
            res += Self::dp(i, n);
        }
        res
    }

    fn dp(i: i32, n: usize) -> i32 {
        if n == 1 {
            1
        } else {
            let mut res = 0;
            for j in 0..=i {
                res += Self::dp(j, n - 1);
            }
            res
        }
    }
}

#[test]
fn test() {
    let n = 1;
    let res = 5;
    assert_eq!(Solution::count_vowel_strings(n), res);
    let n = 2;
    let res = 15;
    assert_eq!(Solution::count_vowel_strings(n), res);
    let n = 33;
    let res = 66045;
    assert_eq!(Solution::count_vowel_strings(n), res);
}
