struct TicTacToe {
    rows: [Vec<usize>; 2],
    cols: [Vec<usize>; 2],
    diagonals: [[usize; 2]; 2],
    n: usize,
}

impl TicTacToe {
    fn new(n: i32) -> Self {
        let n = n as usize;
        let rows = [vec![0; n], vec![0; n]];
        let cols = [vec![0; n], vec![0; n]];
        let diagonals = [[0, 0], [0, 0]];
        TicTacToe {
            rows,
            cols,
            diagonals,
            n,
        }
    }
    fn make_a_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        let i = row as usize;
        let j = col as usize;
        let p = player as usize - 1;
        self.rows[p][i] += 1;
        if self.rows[p][i] == self.n {
            return (p + 1) as i32;
        }
        self.cols[p][j] += 1;
        if self.cols[p][j] == self.n {
            return (p + 1) as i32;
        }
        if i == j {
            self.diagonals[p][0] += 1;
            if self.diagonals[p][0] == self.n {
                return (p + 1) as i32;
            }
        }
        if i + j == self.n - 1 {
            self.diagonals[p][1] += 1;
            if self.diagonals[p][1] == self.n {
                return (p + 1) as i32;
            }
        }
        0
    }
}

#[test]
fn test() {
    let mut toe = TicTacToe::new(3);
    assert_eq!(toe.make_a_move(0, 0, 1), 0);
    assert_eq!(toe.make_a_move(0, 2, 2), 0);
    assert_eq!(toe.make_a_move(2, 2, 1), 0);
    assert_eq!(toe.make_a_move(1, 1, 2), 0);
    assert_eq!(toe.make_a_move(2, 0, 1), 0);
    assert_eq!(toe.make_a_move(1, 0, 2), 0);
    assert_eq!(toe.make_a_move(2, 1, 1), 1);
}
