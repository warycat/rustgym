struct Solution;

impl Solution {
    fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        let n = colors.len();
        let mut color_indexes: Vec<Vec<usize>> = vec![vec![]; 3];
        for i in 0..n {
            let c = colors[i] as usize - 1;
            color_indexes[c].push(i);
        }
        for q in queries {
            let i = q[0] as usize;
            let c = q[1] as usize - 1;
            let indexes = &color_indexes[c];
            match indexes.binary_search(&i) {
                Ok(_) => {
                    res.push(0);
                }
                Err(j) => {
                    let mut min = std::usize::MAX;
                    if j > 0 {
                        min = min.min(i - indexes[j - 1]);
                    }
                    if j < indexes.len() {
                        min = min.min(indexes[j] - i);
                    }
                    if min == std::usize::MAX {
                        res.push(-1);
                    } else {
                        res.push(min as i32);
                    }
                }
            }
        }
        res
    }
}
#[test]
fn test() {
    let colors = vec![1, 1, 2, 1, 3, 2, 2, 3, 3];
    let queries = vec_vec_i32![[1, 3], [2, 2], [6, 1]];
    let res = vec![3, 0, 3];
    assert_eq!(Solution::shortest_distance_color(colors, queries), res);
    let colors = vec![1, 2];
    let queries = vec_vec_i32![[0, 3]];
    let res = vec![-1];
    assert_eq!(Solution::shortest_distance_color(colors, queries), res);
}
