struct Solution;

impl Solution {
    fn find_lonely_pixel(pictures: Vec<Vec<char>>) -> i32 {
        let n = pictures.len();
        let m = pictures[0].len();
        let mut rows = vec![0; n];
        let mut cols = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                if pictures[i][j] == 'B' {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                if pictures[i][j] == 'B' && rows[i] == 1 && cols[j] == 1 {
                    res += 1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let pictures = vec_vec_char![['W', 'W', 'B'], ['W', 'B', 'W'], ['B', 'W', 'W']];
    let res = 3;
    assert_eq!(Solution::find_lonely_pixel(pictures), res);
}
