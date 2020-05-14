struct Solution;

impl Solution {
    fn flip_and_invert_image(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = a.len();
        for i in 0..n {
            let mut l = 0;
            let mut r = n - 1;
            while l < r {
                a[i].swap(l, r);
                l += 1;
                r -= 1;
            }
            for j in 0..n {
                a[i][j] = 1 - a[i][j];
            }
        }
        a
    }
}

#[test]
fn test() {
    let a: Vec<Vec<i32>> = vec_vec_i32![[1, 1, 0], [1, 0, 1], [0, 0, 0]];
    let b: Vec<Vec<i32>> = vec_vec_i32![[1, 0, 0], [0, 1, 0], [1, 1, 1]];
    assert_eq!(Solution::flip_and_invert_image(a), b);
    let a: Vec<Vec<i32>> = vec_vec_i32![[1, 1, 0, 0], [1, 0, 0, 1], [0, 1, 1, 1], [1, 0, 1, 0]];
    let b: Vec<Vec<i32>> = vec_vec_i32![[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 1], [1, 0, 1, 0]];
    assert_eq!(Solution::flip_and_invert_image(a), b);
}
