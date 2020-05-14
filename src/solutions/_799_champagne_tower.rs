struct Solution;

impl Solution {
    fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let query_row = query_row as usize;
        let query_glass = query_glass as usize;
        let mut a: Vec<Vec<f64>> = vec![vec![0.0; 101]; 101];
        a[0][0] = poured as f64;
        for i in 0..99 {
            for j in 0..=i {
                if a[i][j] > 1.0 {
                    let overflow = a[i][j] - 1.0;
                    a[i + 1][j] += 0.5 * overflow;
                    a[i + 1][j + 1] += 0.5 * overflow;
                    a[i][j] = 1.0;
                }
            }
        }
        a[query_row as usize][query_glass as usize]
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let poured = 1;
    let query_glass = 1;
    let query_row = 1;
    let res = 0.0;
    assert_approx_eq!(
        Solution::champagne_tower(poured, query_row, query_glass),
        res
    );
    let poured = 2;
    let query_glass = 1;
    let query_row = 1;
    let res = 0.5;
    assert_approx_eq!(
        Solution::champagne_tower(poured, query_row, query_glass),
        res
    );
    let poured = 0;
    let query_glass = 2;
    let query_row = 0;
    let res = 0.0;
    assert_approx_eq!(
        Solution::champagne_tower(poured, query_row, query_glass),
        res
    );
    let poured = 1;
    let query_glass = 0;
    let query_row = 0;
    let res = 1.0;
    assert_approx_eq!(
        Solution::champagne_tower(poured, query_row, query_glass),
        res
    );
    let poured = 1_000_000_000;
    let query_glass = 99;
    let query_row = 99;
    let res = 0.0;
    assert_approx_eq!(
        Solution::champagne_tower(poured, query_row, query_glass),
        res
    );
}
