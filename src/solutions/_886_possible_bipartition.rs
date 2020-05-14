struct Solution;

impl Solution {
    fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for dislike in dislikes {
            let u = dislike[0] as usize - 1;
            let v = dislike[1] as usize - 1;
            graph[u].push(v);
            graph[v].push(u);
        }
        let mut colors: Vec<i32> = vec![0; n];
        for i in 0..n {
            if colors[i] == 0 && !Self::dfs(i, 1, &mut colors, &graph, n) {
                return false;
            }
        }
        true
    }

    fn dfs(u: usize, color: i32, colors: &mut [i32], graph: &[Vec<usize>], n: usize) -> bool {
        colors[u] = color;
        for &v in &graph[u] {
            if colors[v] == color {
                return false;
            }
            if colors[v] == 0 && !Self::dfs(v, -color, colors, graph, n) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let n = 4;
    let dislikes = vec_vec_i32![[1, 2], [1, 3], [2, 4]];
    let res = true;
    assert_eq!(Solution::possible_bipartition(n, dislikes), res);
    let n = 3;
    let dislikes = vec_vec_i32![[1, 2], [1, 3], [2, 3]];
    let res = false;
    assert_eq!(Solution::possible_bipartition(n, dislikes), res);
    let n = 5;
    let dislikes = vec_vec_i32![[1, 2], [2, 3], [3, 4], [4, 5], [1, 5]];
    let res = false;
    assert_eq!(Solution::possible_bipartition(n, dislikes), res);
}
