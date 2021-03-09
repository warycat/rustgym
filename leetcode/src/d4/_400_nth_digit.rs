struct Solution;

impl Solution {
    fn find_nth_digit(n: i32) -> i32 {
        let mut size = 1;
        let mut start = 1;
        let mut count = 9;
        let mut n = n as usize;
        while n > size * count {
            n -= size * count;
            size += 1;
            start *= 10;
            count *= 10;
        }
        let x = start + (n - 1) / size;
        let v: Vec<i32> = format!("{}", x)
            .bytes()
            .map(|b| (b - b'0') as i32)
            .collect();
        v[(n - 1) % size]
    }
}
#[test]
fn test() {
    let n = 3;
    let res = 3;
    assert_eq!(Solution::find_nth_digit(n), res);
    let n = 11;
    let res = 0;
    assert_eq!(Solution::find_nth_digit(n), res);
}
