struct Solution;

impl Solution {
    fn min_domino_rotations(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut count: Vec<Vec<usize>> = vec![vec![0; 3]; 6];
        let n = a.len();
        for i in 0..n {
            let x = (a[i] - 1) as usize;
            let y = (b[i] - 1) as usize;
            count[x][0] += 1;
            count[y][1] += 1;
            if x == y {
                count[x][2] += 1;
            }
        }
        for i in 0..6 {
            if count[i][0] + count[i][1] - count[i][2] >= n {
                return (usize::min(count[i][0], count[i][1]) - count[i][2]) as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let a = vec![2, 1, 2, 4, 2, 2];
    let b = vec![5, 2, 6, 2, 3, 2];
    let res = 2;
    assert_eq!(Solution::min_domino_rotations(a, b), res);
    let a = vec![3, 5, 1, 2, 3];
    let b = vec![3, 6, 3, 3, 4];
    let res = -1;
    assert_eq!(Solution::min_domino_rotations(a, b), res);
}
