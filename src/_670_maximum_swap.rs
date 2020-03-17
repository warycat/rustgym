struct Solution;

impl Solution {
    fn maximum_swap(num: i32) -> i32 {
        let mut s: Vec<char> = num.to_string().chars().collect();
        let n = s.len();
        let mut last: Vec<usize> = vec![0; 10];
        for i in 0..n {
            last[(s[i] as u8 - b'0') as usize] = i;
        }
        for i in 0..n {
            let d = (s[i] as u8 - b'0') as usize;
            for j in (d + 1..10).rev() {
                if last[j] > i {
                    s.swap(i, last[j]);
                    return s.into_iter().collect::<String>().parse::<i32>().unwrap();
                }
            }
        }
        num
    }
}

#[test]
fn test() {
    let num = 2736;
    let res = 7236;
    assert_eq!(Solution::maximum_swap(num), res);
    let num = 9973;
    let res = 9973;
    assert_eq!(Solution::maximum_swap(num), res);
    let num = 9973;
    let res = 9973;
    assert_eq!(Solution::maximum_swap(num), res);
    let num = 19;
    let res = 91;
    assert_eq!(Solution::maximum_swap(num), res);
}
