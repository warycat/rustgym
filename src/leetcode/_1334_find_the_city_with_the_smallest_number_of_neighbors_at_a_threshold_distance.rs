struct Solution;

impl Solution {
    fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut dist = vec![vec![std::i32::MAX >> 2; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
        }
        for e in edges {
            let i = e[0] as usize;
            let j = e[1] as usize;
            let d = e[2];
            dist[i][j] = d;
            dist[j][i] = d;
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
        let mut min = (n, 0);
        for i in 0..n {
            let count = dist[i].iter().filter(|&&d| d <= distance_threshold).count() - 1;
            if count <= min.0 {
                min = (count, i);
            }
        }
        min.1 as i32
    }
}

#[test]
fn test() {
    let n = 4;
    let edges = vec_vec_i32![[0, 1, 3], [1, 2, 1], [1, 3, 4], [2, 3, 1]];
    let distance_threshold = 4;
    let res = 3;
    assert_eq!(Solution::find_the_city(n, edges, distance_threshold), res);
    let n = 5;
    let edges = vec_vec_i32![
        [0, 1, 2],
        [0, 4, 8],
        [1, 2, 3],
        [1, 4, 2],
        [2, 3, 1],
        [3, 4, 1]
    ];
    let distance_threshold = 2;
    let res = 0;
    assert_eq!(Solution::find_the_city(n, edges, distance_threshold), res);
}
