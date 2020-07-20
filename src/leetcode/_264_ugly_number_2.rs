struct Solution;

impl Solution {
    fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut ugly: Vec<i32> = vec![1];
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while ugly.len() <= n {
            let min = vec![ugly[i] * 2, ugly[j] * 3, ugly[k] * 5]
                .into_iter()
                .min()
                .unwrap();
            ugly.push(min);
            if ugly[i] * 2 == min {
                i += 1;
            }
            if ugly[j] * 3 == min {
                j += 1;
            }
            if ugly[k] * 5 == min {
                k += 1;
            }
        }
        ugly[n - 1]
    }
}

#[test]
fn test() {
    let n = 10;
    let res = 12;
    assert_eq!(Solution::nth_ugly_number(n), res);
}
