
struct Solution;

impl Solution {
    fn candy_crush(mut board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = board.len();
        let m = board[0].len();
        let mut found = true;
        while found {
            found = false;
            for i in 0..n {
                for j in 0..m {
                    let val = board[i][j].abs();
                    if val == 0 {
                        continue;
                    }
                    if j + 2 < m && board[i][j + 1].abs() == val && board[i][j + 2].abs() == val {
                        found = true;
                        let mut k = j;
                        while k < m && board[i][k].abs() == val {
                            board[i][k] = -val;
                            k += 1;
                        }
                    }
                    if i + 2 < n && board[i + 1][j].abs() == val && board[i + 2][j].abs() == val {
                        found = true;
                        let mut k = i;
                        while k < n && board[k][j].abs() == val {
                            board[k][j] = -val;
                            k += 1;
                        }
                    }
                }
            }

            for j in 0..m {
                let mut k = n;
                for i in (0..n).rev() {
                    if board[i][j] > 0 {
                        board[k - 1][j] = board[i][j];
                        k -= 1;
                    }
                }
                for i in 0..k {
                    board[i][j] = 0;
                }
            }
        }
        board
    }
}

#[test]
fn test() {
    let board: Vec<Vec<i32>> = [
        [110, 5, 112, 113, 114],
        [210, 211, 5, 213, 214],
        [310, 311, 3, 313, 314],
        [410, 411, 412, 5, 414],
        [5, 1, 512, 3, 3],
        [610, 4, 1, 613, 614],
        [710, 1, 2, 713, 714],
        [810, 1, 2, 1, 1],
        [1, 1, 2, 2, 2],
        [4, 1, 4, 4, 1014],
    ]
    .iter()
    .map(|v| v.to_vec())
    .collect();
    let res: Vec<Vec<i32>> = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [110, 0, 0, 0, 114],
        [210, 0, 0, 0, 214],
        [310, 0, 0, 113, 314],
        [410, 0, 0, 213, 414],
        [610, 211, 112, 313, 614],
        [710, 311, 412, 613, 714],
        [810, 411, 512, 713, 1014],
    ]
    .iter()
    .map(|v| v.to_vec())
    .collect();
    assert_eq!(Solution::candy_crush(board), res);
}
