struct Solution;

impl Solution {
    fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
        let n = row.len();
        let mut res = 0;
        for i in 0..n {
            if i % 2 == 1 {
                continue;
            }
            if row[i] == row[i + 1] ^ 1 {
                continue;
            }
            res += 1;
            for j in i + 2..n {
                if row[i] == row[j] ^ 1 {
                    row.swap(i + 1, j);
                    break;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let row = vec![0, 2, 1, 3];
    let res = 1;
    assert_eq!(Solution::min_swaps_couples(row), res);
    let row = vec![3, 2, 0, 1];
    let res = 0;
    assert_eq!(Solution::min_swaps_couples(row), res);
}
