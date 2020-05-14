struct Solution;

impl Solution {
    fn num_moves_stones_ii(mut stones: Vec<i32>) -> Vec<i32> {
        stones.sort_unstable();
        let n = stones.len();
        let mut min = n as i32;
        let mut max = 0;
        max = max.max(stones[n - 1] - stones[1] + 1 - n as i32 + 1);
        max = max.max(stones[n - 2] - stones[0] + 1 - n as i32 + 1);
        let mut l = 0;
        for r in 0..n {
            while stones[r] - stones[l] + 1 > n as i32 {
                l += 1;
            }
            let d = r - l + 1;
            if d == n - 1 && stones[r] - stones[l] + 1 == n as i32 - 1 {
                min = min.min(2);
            } else {
                min = min.min((n - d) as i32);
            }
        }
        vec![min, max]
    }
}

#[test]
fn test() {
    let stones = vec![7, 4, 9];
    let res = vec![1, 2];
    assert_eq!(Solution::num_moves_stones_ii(stones), res);
    let stones = vec![6, 5, 4, 3, 10];
    let res = vec![2, 3];
    assert_eq!(Solution::num_moves_stones_ii(stones), res);
    let stones = vec![100, 101, 104, 102, 103];
    let res = vec![0, 0];
    assert_eq!(Solution::num_moves_stones_ii(stones), res);
}
