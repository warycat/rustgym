struct Solution;

type Point = (i32, i32);

#[derive(Debug)]
struct Chessboard {
    board: Vec<Vec<f64>>,
    temp: Vec<Vec<f64>>,
    n: i32,
}

impl Chessboard {
    fn new(n: i32, p: Point) -> Self {
        let m = n as usize;
        let mut board: Vec<Vec<f64>> = vec![vec![0.0; m]; m];
        let temp: Vec<Vec<f64>> = vec![vec![0.0; m]; m];
        board[p.0 as usize][p.1 as usize] = 1.0;
        Chessboard { board, temp, n }
    }
    fn adj(&self, p: Point) -> Vec<Point> {
        let offsets = [
            (1, 2),
            (2, 1),
            (2, -1),
            (1, -2),
            (-1, -2),
            (-2, -1),
            (-2, 1),
            (-1, 2),
        ];
        let mut res: Vec<Point> = vec![];
        for offset in &offsets {
            let i = p.0 + offset.0;
            let j = p.1 + offset.1;
            if i < 0 || i >= self.n || j < 0 || j >= self.n {
                continue;
            }
            res.push((i, j));
        }
        res
    }
    fn next(&mut self) {
        let m = self.n as usize;
        for i in 0..m {
            for j in 0..m {
                let f = self.board[i][j];
                if f == 0.0 {
                    continue;
                }
                for p in self.adj((i as i32, j as i32)) {
                    self.temp[p.0 as usize][p.1 as usize] += f;
                }
            }
        }
        for i in 0..m {
            for j in 0..m {
                self.board[i][j] = self.temp[i][j] * 0.125;
                self.temp[i][j] = 0.0;
            }
        }
    }
    fn sum(&mut self) -> f64 {
        let mut res = 0.0;
        let m = self.n as usize;
        for i in 0..m {
            for j in 0..m {
                res += self.board[i][j];
            }
        }
        res
    }
}

impl Solution {
    fn knight_probability(n: i32, k: i32, r: i32, c: i32) -> f64 {
        let mut chessboard: Chessboard = Chessboard::new(n, (r, c));
        for _ in 0..k {
            chessboard.next();
        }
        chessboard.sum()
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let n = 3;
    let k = 2;
    let r = 0;
    let c = 0;
    let res = 0.0625;
    assert_approx_eq!(Solution::knight_probability(n, k, r, c), res);
    let n = 3;
    let k = 1;
    let r = 1;
    let c = 1;
    let res = 0.0;
    assert_approx_eq!(Solution::knight_probability(n, k, r, c), res);
}
