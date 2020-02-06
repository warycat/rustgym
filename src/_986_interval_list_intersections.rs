struct Solution;

impl Solution {
    fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut i: usize = 0;
        let mut j: usize = 0;
        let n = a.len();
        let m = b.len();
        while i < n && j < m {
            let al = a[i][0];
            let ar = a[i][1];
            let bl = b[j][0];
            let br = b[j][1];
            if ar < bl {
                i += 1;
                continue;
            }
            if br < al {
                j += 1;
                continue;
            }
            let l = i32::max(al, bl);
            let r = i32::min(ar, br);
            res.push(vec![l, r]);
            if ar < br {
                i += 1;
            } else {
                j += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let a: Vec<Vec<i32>> = vec_vec_i32![[0, 2], [5, 10], [13, 23], [24, 25]];
    let b: Vec<Vec<i32>> = vec_vec_i32![[1, 5], [8, 12], [15, 24], [25, 26]];
    let res: Vec<Vec<i32>> = vec_vec_i32![[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]];
    assert_eq!(Solution::interval_intersection(a, b), res);
}
