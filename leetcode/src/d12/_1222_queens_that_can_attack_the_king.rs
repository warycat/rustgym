struct Solution;
use std::collections::HashSet;

type Point = Vec<i32>;

struct Chessboard {
    directions: Vec<Point>,
    queens: HashSet<Point>,
    king: Point,
}

impl Chessboard {
    fn new(queens_vec: Vec<Point>, king: Point) -> Self {
        let directions: Vec<Point> = vec![
            vec![1, 0],
            vec![-1, 0],
            vec![0, 1],
            vec![0, -1],
            vec![1, 1],
            vec![-1, 1],
            vec![1, -1],
            vec![-1, -1],
        ];
        let mut queens: HashSet<Point> = HashSet::new();
        for queen in queens_vec {
            queens.insert(queen);
        }
        Chessboard {
            directions,
            queens,
            king,
        }
    }
    fn contains(&self, point: &[i32]) -> bool {
        point[0] >= 0 && point[1] >= 0 && point[0] < 8 && point[1] < 8
    }
    fn attack(&self, i: usize, step: i32) -> Point {
        let direction = &self.directions[i];
        let king = &self.king;
        vec![king[0] + direction[0] * step, king[1] + direction[1] * step]
    }
}

impl Solution {
    fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let cb = Chessboard::new(queens, king);
        for i in 0..8 {
            let mut step = 1;
            loop {
                let p = cb.attack(i, step);
                if cb.contains(&p) {
                    if cb.queens.contains(&p) {
                        res.push(p);
                        break;
                    } else {
                        step += 1;
                    }
                } else {
                    break;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let queens = vec_vec_i32![[0, 1], [1, 0], [4, 0], [0, 4], [3, 3], [2, 4]];
    let king = vec![0, 0];
    let mut res = vec_vec_i32![[0, 1], [1, 0], [3, 3]];
    let mut ans = Solution::queens_attackthe_king(queens, king);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let queens = vec_vec_i32![[0, 0], [1, 1], [2, 2], [3, 4], [3, 5], [4, 4], [4, 5]];
    let king = vec![3, 3];
    let mut res = vec_vec_i32![[2, 2], [3, 4], [4, 4]];
    let mut ans = Solution::queens_attackthe_king(queens, king);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let queens = vec_vec_i32![
        [5, 6],
        [7, 7],
        [2, 1],
        [0, 7],
        [1, 6],
        [5, 1],
        [3, 7],
        [0, 3],
        [4, 0],
        [1, 2],
        [6, 3],
        [5, 0],
        [0, 4],
        [2, 2],
        [1, 1],
        [6, 4],
        [5, 4],
        [0, 0],
        [2, 6],
        [4, 5],
        [5, 2],
        [1, 4],
        [7, 5],
        [2, 3],
        [0, 5],
        [4, 2],
        [1, 0],
        [2, 7],
        [0, 1],
        [4, 6],
        [6, 1],
        [0, 6],
        [4, 3],
        [1, 7]
    ];
    let king = vec![3, 4];
    let mut res = vec_vec_i32![[2, 3], [1, 4], [1, 6], [3, 7], [4, 3], [5, 4], [4, 5]];
    let mut ans = Solution::queens_attackthe_king(queens, king);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
