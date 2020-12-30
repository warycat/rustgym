struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut queue: VecDeque<i32> = VecDeque::from(students);
        let mut i = 0;
        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let x = queue.pop_front().unwrap();
                if x == sandwiches[i] {
                    i += 1;
                } else {
                    queue.push_back(x);
                }
            }
            if queue.len() == n {
                break;
            }
        }
        queue.len() as i32
    }
}

#[test]
fn test() {
    let students = vec![1, 1, 0, 0];
    let sandwiches = vec![0, 1, 0, 1];
    let res = 0;
    assert_eq!(Solution::count_students(students, sandwiches), res);
    let students = vec![1, 1, 1, 0, 0, 1];
    let sandwiches = vec![1, 0, 0, 0, 1, 1];
    let res = 3;
    assert_eq!(Solution::count_students(students, sandwiches), res);
}
