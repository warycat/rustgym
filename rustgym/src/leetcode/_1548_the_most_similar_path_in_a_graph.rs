struct Solution;

impl Solution {
    fn most_similar(
        n: i32,
        roads: Vec<Vec<i32>>,
        names: Vec<String>,
        target_path: Vec<String>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for road in roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }
        let m = target_path.len();
        let mut dp = vec![vec![std::usize::MAX; n]; m];
        let mut prev = vec![vec![std::usize::MAX; n]; m];
        for u in 0..n {
            dp[0][u] = if target_path[0] == names[u] { 0 } else { 1 };
        }
        for i in 1..m {
            for u in 0..n {
                for &v in &adj[u] {
                    let dist = dp[i - 1][v] + if target_path[i] == names[u] { 0 } else { 1 };
                    if dist < dp[i][u] {
                        dp[i][u] = dist;
                        prev[i][u] = v;
                    }
                }
            }
        }
        let mut res = vec![std::usize::MAX; m];
        let mut min = std::usize::MAX;
        for i in 0..n {
            if dp[m - 1][i] < min {
                res[m - 1] = i;
                min = dp[m - 1][i];
            }
        }
        for i in (1..m).rev() {
            res[i - 1] = prev[i][res[i]];
        }
        res.into_iter().map(|x| x as i32).collect()
    }
}

#[test]
fn test() {
    let n = 5;
    let roads = vec_vec_i32![[0, 2], [0, 3], [1, 2], [1, 3], [1, 4], [2, 4]];
    let names = vec_string!["ATL", "PEK", "LAX", "DXB", "HND"];
    let target_path = vec_string!["ATL", "DXB", "HND", "LAX"];
    let res = vec![0, 3, 0, 2];
    assert_eq!(Solution::most_similar(n, roads, names, target_path), res);
    let n = 4;
    let roads = vec_vec_i32![[1, 0], [2, 0], [3, 0], [2, 1], [3, 1], [3, 2]];
    let names = vec_string!["ATL", "PEK", "LAX", "DXB"];
    let target_path = vec_string!["ABC", "DEF", "GHI", "JKL", "MNO", "PQR", "STU", "VWX"];
    let res = vec![1, 0, 1, 0, 1, 0, 1, 0];
    assert_eq!(Solution::most_similar(n, roads, names, target_path), res);
    let n = 6;
    let roads = vec_vec_i32![[0, 1], [1, 2], [2, 3], [3, 4], [4, 5]];
    let names = vec_string!["ATL", "PEK", "LAX", "ATL", "DXB", "HND"];
    let target_path = vec_string!["ATL", "DXB", "HND", "DXB", "ATL", "LAX", "PEK"];
    let res = vec![3, 4, 5, 4, 3, 2, 1];
    assert_eq!(Solution::most_similar(n, roads, names, target_path), res);
    let n = 4;
    let roads = vec_vec_i32![[1, 3], [3, 0], [1, 0], [0, 2], [2, 1]];
    let names = vec_string!["HSV", "HSV", "HSV", "FAI"];
    let target_path = vec_string![
        "HSV", "HSV", "HSV", "HSV", "HSV", "FAI", "HSV", "HSV", "HSV", "HSV", "FAI", "FAI", "HSV",
        "FAI", "HSV", "HSV", "HSV", "HSV", "HSV", "HSV", "FAI", "HSV", "HSV", "HSV", "HSV", "HSV",
        "HSV", "HSV", "HSV", "HSV", "HSV"
    ];
    let res = vec![
        1, 0, 1, 0, 1, 3, 1, 0, 1, 0, 1, 3, 1, 3, 0, 1, 0, 1, 0, 1, 3, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
    ];
    assert_eq!(Solution::most_similar(n, roads, names, target_path), res);
}
