struct Solution;

#[derive(Debug, PartialEq, Eq, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    fn from(v: Vec<i32>) -> Self {
        Point { x: v[0], y: v[1] }
    }
    fn square_distance(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

#[derive(Default)]
struct Robot {
    position: Point,
    direction: usize,
}

impl Robot {
    fn new() -> Self {
        Robot {
            position: Point { x: 0, y: 0 },
            direction: 0,
        }
    }
    fn left(&mut self) {
        self.direction = (self.direction + 1) % 4;
    }
    fn right(&mut self) {
        self.direction = (self.direction + 3) % 4;
    }
    fn next(&self) -> Point {
        let x = self.position.x;
        let y = self.position.y;
        match self.direction {
            0 => Point::new(x, y + 1),
            1 => Point::new(x - 1, y),
            2 => Point::new(x, y - 1),
            3 => Point::new(x + 1, y),
            _ => unreachable!(),
        }
    }
    fn walk(&mut self, step: usize, grid: &Grid) {
        let mut i = 0;
        while i < step && !grid.obstacles.contains(&self.next()) {
            self.position = self.next();
            i += 1;
        }
    }
}

use std::collections::HashSet;

struct Grid {
    obstacles: HashSet<Point>,
}

impl Grid {
    fn new(obstacles: Vec<Point>) -> Self {
        let mut hs: HashSet<Point> = HashSet::new();
        for x in obstacles {
            hs.insert(x);
        }
        Grid { obstacles: hs }
    }
}

impl Solution {
    fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let grid: Grid = Grid::new(obstacles.iter().map(|v| Point::new(v[0], v[1])).collect());
        let mut robot = Robot::new();
        let mut max = 0;
        for command in commands {
            match command {
                -2 => robot.left(),
                -1 => robot.right(),
                _ => {
                    robot.walk(command as usize, &grid);
                    max = i32::max(robot.position.square_distance(), max);
                }
            }
        }
        max as i32
    }
}

#[test]
fn test() {
    let commands: Vec<i32> = vec![4, -1, 3];
    let obstacles: Vec<Vec<i32>> = vec![];
    assert_eq!(Solution::robot_sim(commands, obstacles), 25);
    let commands: Vec<i32> = vec![4, -1, 4, -2, 4];
    let obstacles: Vec<Vec<i32>> = vec![vec![2, 4]];
    assert_eq!(Solution::robot_sim(commands, obstacles), 65);
}
