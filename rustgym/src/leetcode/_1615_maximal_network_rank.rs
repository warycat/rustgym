struct Solution;

impl Solution {
    fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj: Vec<Vec<bool>> = vec![vec![false; n]; n];
        let mut size: Vec<usize> = vec![0; n];
        for road in roads {
            let i = road[0] as usize;
            let j = road[1] as usize;
            adj[i][j] = true;
            adj[j][i] = true;
            size[i] += 1;
            size[j] += 1;
        }
        let mut res = 0;
        for i in 0..n {
            for j in i + 1..n {
                res = res.max(size[i] + size[j] - if adj[i][j] { 1 } else { 0 });
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let n = 4;
    let roads = vec_vec_i32![[0, 1], [0, 3], [1, 2], [1, 3]];
    let res = 4;
    assert_eq!(Solution::maximal_network_rank(n, roads), res);
    let n = 5;
    let roads = vec_vec_i32![[0, 1], [0, 3], [1, 2], [1, 3], [2, 3], [2, 4]];
    let res = 5;
    assert_eq!(Solution::maximal_network_rank(n, roads), res);
    let n = 8;
    let roads = vec_vec_i32![[0, 1], [1, 2], [2, 3], [2, 4], [5, 6], [5, 7]];
    let res = 5;
    assert_eq!(Solution::maximal_network_rank(n, roads), res);
}
