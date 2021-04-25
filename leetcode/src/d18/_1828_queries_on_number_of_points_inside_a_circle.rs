struct Solution;

impl Solution {
    fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries.into_iter().map(|q| query(&points, q)).collect()
    }
}

fn query(points: &[Vec<i32>], q: Vec<i32>) -> i32 {
    let r2 = q[2] * q[2];
    let mut res = 0;
    for p in points {
        let dx = p[0] - q[0];
        let dy = p[1] - q[1];
        if dx * dx + dy * dy <= r2 {
            res += 1;
        }
    }
    res
}

#[test]
fn test() {
    let points = vec_vec_i32![[1, 3], [3, 3], [5, 3], [2, 2]];
    let queries = vec_vec_i32![[2, 3, 1], [4, 3, 1], [1, 1, 2]];
    let res = vec![3, 2, 2];
    assert_eq!(Solution::count_points(points, queries), res);

    let points = vec_vec_i32![[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]];
    let queries = vec_vec_i32![[1, 2, 2], [2, 2, 2], [4, 3, 2], [4, 3, 3]];
    let res = vec![2, 3, 2, 4];
    assert_eq!(Solution::count_points(points, queries), res);
}
