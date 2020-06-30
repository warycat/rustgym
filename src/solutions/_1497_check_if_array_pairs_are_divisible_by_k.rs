struct Solution;

impl Solution {
    fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut count = vec![0; k as usize];
        for x in arr {
            let y = x % k;
            let y = if y < 0 { y + k } else { y };
            count[y as usize] += 1;
        }
        if count[0] % 2 != 0 {
            return false;
        }
        let k = k as usize;
        for i in 1..k {
            if count[i] != count[k - i] {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let arr = vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9];
    let k = 5;
    let res = true;
    assert_eq!(Solution::can_arrange(arr, k), res);
    let arr = vec![1, 2, 3, 4, 5, 6];
    let k = 7;
    let res = true;
    assert_eq!(Solution::can_arrange(arr, k), res);
    let arr = vec![1, 2, 3, 4, 5, 6];
    let k = 10;
    let res = false;
    assert_eq!(Solution::can_arrange(arr, k), res);
    let arr = vec![-10, 10];
    let k = 2;
    let res = true;
    assert_eq!(Solution::can_arrange(arr, k), res);
    let arr = vec![-1, 1, -2, 2, -3, 3, -4, 4];
    let k = 3;
    let res = true;
    assert_eq!(Solution::can_arrange(arr, k), res);
}
