struct Solution;

impl Solution {
    fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        let n = tasks.len();
        let mut sum = 0;
        let mut min = std::i32::MAX;
        let mut max = 0;
        for i in 0..n {
            sum += tasks[i][0];
            min = min.min(tasks[i][1] - tasks[i][0]);
            max = max.max(tasks[i][1]);
        }
        (sum + min).max(max)
    }
}

#[test]
fn test() {
    let tasks = vec_vec_i32![[1, 2], [2, 4], [4, 8]];
    let res = 8;
    assert_eq!(Solution::minimum_effort(tasks), res);
    let tasks = vec_vec_i32![[1, 3], [2, 4], [10, 11], [10, 12], [8, 9]];
    let res = 32;
    assert_eq!(Solution::minimum_effort(tasks), res);
    let tasks = vec_vec_i32![[1, 7], [2, 8], [3, 9], [4, 10], [5, 11], [6, 12]];
    let res = 27;
    assert_eq!(Solution::minimum_effort(tasks), res);
    let tasks = vec_vec_i32![[1, 1], [1, 3]];
    let res = 3;
    assert_eq!(Solution::minimum_effort(tasks), res);
}
