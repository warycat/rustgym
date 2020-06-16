struct Solution;

impl Solution {
    fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let m = m;
        let k = k as usize;
        let n = bloom_day.len();
        if n < m as usize * k {
            return -1;
        }
        let mut left = 1;
        let mut right = std::i32::MAX;
        while left < right {
            let mid = left + (right - left) / 2;
            let mut group = 0;
            let mut count = 0;
            for i in 0..n {
                if bloom_day[i] > mid {
                    count = 0;
                } else {
                    count += 1;
                    if count >= k {
                        count = 0;
                        group += 1;
                    }
                }
            }
            if group < m {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

#[test]
fn test() {
    let bloom_day = vec![1, 10, 3, 10, 2];
    let m = 3;
    let k = 1;
    let res = 3;
    assert_eq!(Solution::min_days(bloom_day, m, k), res);
    let bloom_day = vec![1, 10, 3, 10, 2];
    let m = 3;
    let k = 2;
    let res = -1;
    assert_eq!(Solution::min_days(bloom_day, m, k), res);
    let bloom_day = vec![7, 7, 7, 7, 12, 7, 7];
    let m = 2;
    let k = 3;
    let res = 12;
    assert_eq!(Solution::min_days(bloom_day, m, k), res);
    let bloom_day = vec![1000000000, 1000000000];
    let m = 1;
    let k = 1;
    let res = 1000000000;
    assert_eq!(Solution::min_days(bloom_day, m, k), res);
    let bloom_day = vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6];
    let m = 4;
    let k = 2;
    let res = 9;
    assert_eq!(Solution::min_days(bloom_day, m, k), res);
}
