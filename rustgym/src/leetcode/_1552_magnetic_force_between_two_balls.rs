struct Solution;

impl Solution {
    fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();
        let n = position.len();
        let mut low = std::i32::MAX;
        for w in position.windows(2) {
            low = low.min(w[1] - w[0]);
        }
        let mut high = (position[n - 1] - position[0]) / (m - 1) as i32;
        while low < high {
            let mid = (low + high + 1) / 2;
            let mut prev = position[0];
            let mut count = 1;
            for i in 1..n {
                if position[i] - prev >= mid {
                    count += 1;
                    prev = position[i];
                }
            }
            if count < m {
                high = mid - 1;
            } else {
                low = mid;
            }
        }
        low
    }
}

#[test]
fn test() {
    let position = vec![1, 2, 3, 4, 7];
    let m = 3;
    let res = 3;
    assert_eq!(Solution::max_distance(position, m), res);
    let position = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let m = 4;
    let res = 3;
    assert_eq!(Solution::max_distance(position, m), res);
}
