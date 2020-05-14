struct Solution;

impl Solution {
    fn magical_string(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n <= 3 {
            return 1;
        }
        let n = n as usize;
        let mut a = vec![1, 2, 2];
        let mut i = 2;
        let mut x = 1;
        let mut res = 1;
        loop {
            for _ in 0..a[i] {
                if x == 1 {
                    res += 1;
                }
                a.push(x);
                if a.len() >= n {
                    return res;
                }
            }
            x = 3 - x;
            i += 1;
        }
    }
}

#[test]
fn test() {
    let n = 6;
    let res = 3;
    assert_eq!(Solution::magical_string(n), res);
}
