struct Solution;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct Knight {
    point: Point,
    step: i32,
}

impl Knight {
    fn new(point: Point, step: i32) -> Self {
        Knight { point, step }
    }
    fn next(&self) -> Vec<Knight> {
        let offsets: [[i32; 2]; 8] = [
            [2, 1],
            [1, 2],
            [-1, 2],
            [-2, 1],
            [-2, -1],
            [-1, -2],
            [1, -2],
            [2, -1],
        ];
        let mut res: Vec<Knight> = vec![];
        let x = self.point.x;
        let y = self.point.y;
        let step = self.step;
        for offset in offsets.iter() {
            let knight = Knight::new(Point::new(x + offset[0], y + offset[1]), step + 1);
            res.push(knight);
        }
        res
    }
}

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    fn min_knight_moves(x: i32, y: i32) -> i32 {
        let end = Point::new(i32::min(x.abs(), y.abs()), i32::max(x.abs(), y.abs()));
        let mut hs: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<Knight> = VecDeque::new();
        hs.insert(Point::new(0, 0));
        queue.push_back(Knight::new(Point::new(0, 0), 0));
        while let Some(knight) = queue.pop_front() {
            if knight.point == end {
                return knight.step;
            } else {
                let next = knight.next();
                for knight in next {
                    if knight.point.y > 0 && knight.point.x <= knight.point.y {
                        if hs.insert(knight.point) {
                            queue.push_back(knight);
                        }
                    }
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let x = 2;
    let y = 1;
    assert_eq!(Solution::min_knight_moves(x, y), 1);
    let x = 5;
    let y = 5;
    assert_eq!(Solution::min_knight_moves(x, y), 4);
}
