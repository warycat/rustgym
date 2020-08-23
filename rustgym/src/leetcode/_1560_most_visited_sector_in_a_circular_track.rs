struct Solution;

impl Solution {
    fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let m = rounds.len();
        let start = rounds[0];
        let end = rounds[m - 1];
        if start == end {
            return vec![start];
        }
        if start < end {
            return (start..=end).collect();
        }
        let mut res: Vec<i32> = (start..=end + n).map(|x| ((x - 1) % n) + 1).collect();
        res.sort_unstable();
        res
    }
}

#[test]
fn test() {
    let n = 4;
    let rounds = vec![1, 3, 1, 2];
    let res = vec![1, 2];
    assert_eq!(Solution::most_visited(n, rounds), res);
    let n = 2;
    let rounds = vec![2, 1, 2, 1, 2, 1, 2, 1, 2];
    let res = vec![2];
    assert_eq!(Solution::most_visited(n, rounds), res);
    let n = 7;
    let rounds = vec![1, 3, 5, 7];
    let res = vec![1, 2, 3, 4, 5, 6, 7];
    assert_eq!(Solution::most_visited(n, rounds), res);
    let n = 3;
    let rounds = vec![3, 2, 1, 2, 1, 3, 2, 1, 2, 1, 3, 2, 3, 1];
    let res = vec![1, 3];
    assert_eq!(Solution::most_visited(n, rounds), res);
}
