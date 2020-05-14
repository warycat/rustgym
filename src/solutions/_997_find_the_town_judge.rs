struct Solution;

impl Solution {
    fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut degree = vec![0; n];
        for edge in trust {
            let u = (edge[0] - 1) as usize;
            let v = (edge[1] - 1) as usize;
            degree[v] += 1;
            degree[u] -= 1;
        }
        for i in 0..n {
            if degree[i] as usize == n - 1 {
                return (i + 1) as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let n = 2;
    let trust = vec_vec_i32![[1, 2]];
    let res = 2;
    assert_eq!(Solution::find_judge(n, trust), res);
    let n = 3;
    let trust = vec_vec_i32![[1, 3], [2, 3]];
    let res = 3;
    assert_eq!(Solution::find_judge(n, trust), res);
    let n = 3;
    let trust = vec_vec_i32![[1, 3], [2, 3], [3, 1]];
    let res = -1;
    assert_eq!(Solution::find_judge(n, trust), res);
    let n = 3;
    let trust = vec_vec_i32![[1, 2], [2, 3]];
    let res = -1;
    assert_eq!(Solution::find_judge(n, trust), res);
    let n = 4;
    let trust = vec_vec_i32![[1, 3], [1, 4], [2, 3], [2, 4], [4, 3]];
    let res = 3;
    assert_eq!(Solution::find_judge(n, trust), res);
}
