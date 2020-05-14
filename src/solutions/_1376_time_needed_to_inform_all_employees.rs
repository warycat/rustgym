struct Solution;

impl Solution {
    fn num_of_minutes(n: i32, _: i32, mut manager: Vec<i32>, mut inform_time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut res = 0;
        for i in 0..n {
            res = res.max(Self::dfs(i, &mut manager, &mut inform_time));
        }
        res
    }

    fn dfs(i: usize, manager: &mut Vec<i32>, inform_time: &mut Vec<i32>) -> i32 {
        if manager[i] != -1 {
            inform_time[i] += Self::dfs(manager[i] as usize, manager, inform_time);
            manager[i] = -1;
        }
        inform_time[i]
    }
}

#[test]
fn test() {
    let n = 1;
    let head_id = 0;
    let manager = vec![-1];
    let inform_time = vec![0];
    let res = 0;
    assert_eq!(
        Solution::num_of_minutes(n, head_id, manager, inform_time),
        res
    );
    let n = 6;
    let head_id = 2;
    let manager = vec![2, 2, -1, 2, 2, 2];
    let inform_time = vec![0, 0, 1, 0, 0, 0];
    let res = 1;
    assert_eq!(
        Solution::num_of_minutes(n, head_id, manager, inform_time),
        res
    );
    let n = 7;
    let head_id = 6;
    let manager = vec![1, 2, 3, 4, 5, 6, -1];
    let inform_time = vec![0, 6, 5, 4, 3, 2, 1];
    let res = 21;
    assert_eq!(
        Solution::num_of_minutes(n, head_id, manager, inform_time),
        res
    );
    let n = 15;
    let head_id = 0;
    let manager = vec![-1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6];
    let inform_time = vec![1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0];
    let res = 3;
    assert_eq!(
        Solution::num_of_minutes(n, head_id, manager, inform_time),
        res
    );
    let n = 4;
    let head_id = 2;
    let manager = vec![3, 3, -1, 2];
    let inform_time = vec![0, 0, 162, 914];
    let res = 1076;
    assert_eq!(
        Solution::num_of_minutes(n, head_id, manager, inform_time),
        res
    );
}
