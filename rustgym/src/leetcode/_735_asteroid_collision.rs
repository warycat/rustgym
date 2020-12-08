struct Solution;
use std::cmp::Ordering::*;

impl Solution {
    fn collide(stack: &mut Vec<i32>, asteroid: i32) {
        if let Some(&top) = stack.last() {
            if top < 0 {
                stack.push(asteroid);
            } else {
                match top.cmp(&-asteroid) {
                    Less => {
                        stack.pop();
                        Self::collide(stack, asteroid);
                    }
                    Equal => {
                        stack.pop();
                    }
                    Greater => {}
                }
            }
        } else {
            stack.push(asteroid);
        }
    }

    fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        for x in asteroids {
            if x > 0 {
                res.push(x);
            } else {
                Self::collide(&mut res, x);
            }
        }
        res
    }
}

#[test]
fn test() {
    let asteroids: Vec<i32> = vec![5, 10, -5];
    let res = vec![5, 10];
    assert_eq!(Solution::asteroid_collision(asteroids), res);
    let asteroids: Vec<i32> = vec![8, -8];
    let res: Vec<i32> = vec![];
    assert_eq!(Solution::asteroid_collision(asteroids), res);
    let asteroids: Vec<i32> = vec![10, 2, -5];
    let res = vec![10];
    assert_eq!(Solution::asteroid_collision(asteroids), res);
    let asteroids: Vec<i32> = vec![-2, -1, 1, 2];
    let res = vec![-2, -1, 1, 2];
    assert_eq!(Solution::asteroid_collision(asteroids), res);
}
