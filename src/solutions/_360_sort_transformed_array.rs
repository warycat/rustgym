struct Solution;
use std::cmp::Ordering::*;
use std::collections::VecDeque;

impl Solution {
    fn sort_transformed_array(nums: Vec<i32>, a: i32, b: i32, c: i32) -> Vec<i32> {
        match a.cmp(&0) {
            Equal => {
                if b >= 0 {
                    let res: Vec<i32> = nums.into_iter().map(|x| Self::f(x, a, b, c)).collect();
                    res
                } else {
                    let mut res: Vec<i32> = nums.into_iter().map(|x| Self::f(x, a, b, c)).collect();
                    res.reverse();
                    res
                }
            }
            Greater => {
                let mut queue = VecDeque::from(nums);
                let mut res = vec![];
                while queue.len() > 1 {
                    let front = queue.front().unwrap();
                    let back = queue.back().unwrap();
                    let front = Self::f(*front, a, b, c);
                    let back = Self::f(*back, a, b, c);
                    if front > back {
                        queue.pop_front();
                        res.push(front);
                    } else {
                        queue.pop_back();
                        res.push(back);
                    }
                }
                let last = queue.pop_front().unwrap();
                let last = Self::f(last, a, b, c);
                res.push(last);
                res.reverse();
                res
            }
            Less => {
                let mut queue = VecDeque::from(nums);
                let mut res = vec![];
                while queue.len() > 1 {
                    let front = queue.front().unwrap();
                    let back = queue.back().unwrap();
                    let front = Self::f(*front, a, b, c);
                    let back = Self::f(*back, a, b, c);
                    if front < back {
                        queue.pop_front();
                        res.push(front);
                    } else {
                        queue.pop_back();
                        res.push(back);
                    }
                }
                let last = queue.pop_front().unwrap();
                let last = Self::f(last, a, b, c);
                res.push(last);
                res
            }
        }
    }
    fn f(x: i32, a: i32, b: i32, c: i32) -> i32 {
        a * x * x + b * x + c
    }
}

#[test]
fn test() {
    let nums = vec![-4, -2, 2, 4];
    let a = 1;
    let b = 3;
    let c = 5;
    let res = vec![3, 9, 15, 33];
    assert_eq!(Solution::sort_transformed_array(nums, a, b, c), res);
    let nums = vec![-4, -2, 2, 4];
    let a = -1;
    let b = 3;
    let c = 5;
    let res = vec![-23, -5, 1, 7];
    assert_eq!(Solution::sort_transformed_array(nums, a, b, c), res);
}
