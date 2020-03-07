use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Point(i32, i32);

impl Point {
    fn from_direction(direction: String) -> Point {
        if direction == "U" {
            Point(-1, 0)
        } else if direction == "L" {
            Point(0, -1)
        } else if direction == "R" {
            Point(0, 1)
        } else if direction == "D" {
            Point(1, 0)
        } else {
            Point(0, 0)
        }
    }
}

struct SnakeGame {
    body: VecDeque<Point>,
    food: Vec<Point>,
    screen: HashMap<Point, bool>,
    n: i32,
    m: i32,
    score: i32,
}

impl SnakeGame {
    fn new(width: i32, height: i32, food: Vec<Vec<i32>>) -> Self {
        let score = 0;
        let n = height as i32;
        let m = width as i32;
        let mut body: VecDeque<Point> = VecDeque::new();
        let head = Point(0, 0);
        body.push_back(head);
        let mut screen: HashMap<Point, bool> = HashMap::new();
        *screen.entry(Point(0, 0)).or_default() = true;
        let food = food.iter().rev().map(|v| Point(v[0], v[1])).collect();
        SnakeGame {
            body,
            food,
            screen,
            m,
            n,
            score,
        }
    }

    fn make_a_move(&mut self, direction: String) -> i32 {
        let head = self.body.front().unwrap();
        let offset = Point::from_direction(direction);
        let next = Point(head.0 + offset.0, head.1 + offset.1);
        if let Some(food) = self.food.last() {
            if *food == next {
                self.food.pop();
                self.score += 1;
            } else {
                let tail = self.body.pop_back().unwrap();
                *self.screen.entry(tail).or_default() = false;
            }
        }
        if next.0 < 0 || next.0 >= self.n || next.1 < 0 || next.1 >= self.m {
            return -1;
        }
        if *self.screen.entry(next).or_default() {
            return -1;
        }
        *self.screen.entry(next).or_default() = true;
        self.body.push_front(next);
        self.score
    }
}

#[test]
fn test() {
    let mut snake = SnakeGame::new(3, 2, vec_vec_i32![[1, 2], [0, 1]]);
    assert_eq!(snake.make_a_move("R".to_string()), 0);
    assert_eq!(snake.make_a_move("D".to_string()), 0);
    assert_eq!(snake.make_a_move("R".to_string()), 1);
    assert_eq!(snake.make_a_move("U".to_string()), 1);
    assert_eq!(snake.make_a_move("L".to_string()), 2);
    assert_eq!(snake.make_a_move("U".to_string()), -1);
}
