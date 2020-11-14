struct Solution;

impl Solution {
    fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let n = n as usize + 1;
        let mut arr = vec![0; n];
        arr[0] = 0;
        arr[1] = 1;
        for i in 2..n {
            if i % 2 == 0 {
                arr[i] = arr[i / 2];
            } else {
                arr[i] = arr[i / 2] + arr[i / 2 + 1];
            }
        }
        arr.into_iter().max().unwrap()
    }
}

#[test]
fn test() {
    let n = 7;
    let res = 3;
    assert_eq!(Solution::get_maximum_generated(n), res);
    let n = 2;
    let res = 1;
    assert_eq!(Solution::get_maximum_generated(n), res);
    let n = 3;
    let res = 2;
    assert_eq!(Solution::get_maximum_generated(n), res);
}
