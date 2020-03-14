struct Solution;

impl Solution {
    fn find_black_pixel(picture: Vec<Vec<char>>, n: i32) -> i32 {
        let k = n as usize;
        let n = picture.len();
        let m = picture[0].len();
        let mut rows: Vec<Vec<usize>> = vec![vec![]; n];
        let mut cols: Vec<Vec<usize>> = vec![vec![]; m];
        for i in 0..n {
            for j in 0..m {
                if picture[i][j] == 'B' {
                    rows[i].push(j);
                    cols[j].push(i);
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                if picture[i][j] == 'B' && rows[i].len() == k && cols[j].len() == k {
                    if cols[j].iter().all(|&r| rows[r] == rows[i]) {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let picture = vec_vec_char![
        ['W', 'B', 'W', 'B', 'B', 'W'],
        ['W', 'B', 'W', 'B', 'B', 'W'],
        ['W', 'B', 'W', 'B', 'B', 'W'],
        ['W', 'W', 'B', 'W', 'B', 'W']
    ];
    let n = 3;
    let res = 6;
    assert_eq!(Solution::find_black_pixel(picture, n), res);
}
