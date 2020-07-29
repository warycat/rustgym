struct Solution;

impl Solution {
    fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
        let mut left = 1;
        let mut right = 1_000_000_000;
        while left < right {
            let mid = (left + right + 1) / 2;
            let mut cut = 0;
            let mut cur = 0;
            for &x in &sweetness {
                cur += x;
                if cur >= mid {
                    cut += 1;
                    cur = 0;
                }
            }
            if cut > k {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left
    }
}

#[test]
fn test() {
    let sweetness = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let k = 5;
    let res = 6;
    assert_eq!(Solution::maximize_sweetness(sweetness, k), res);
    let sweetness = vec![5, 6, 7, 8, 9, 1, 2, 3, 4];
    let k = 8;
    let res = 1;
    assert_eq!(Solution::maximize_sweetness(sweetness, k), res);
    let sweetness = vec![5, 6, 7, 8, 9, 1, 2, 3, 4];
    let k = 8;
    let res = 1;
    assert_eq!(Solution::maximize_sweetness(sweetness, k), res);
    let sweetness = vec![1, 2, 2, 1, 2, 2, 1, 2, 2];
    let k = 2;
    let res = 5;
    assert_eq!(Solution::maximize_sweetness(sweetness, k), res);
}
