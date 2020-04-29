struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        if z > x + y {
            return false;
        }
        if z == x || z == y || z == x + y {
            return true;
        }
        let mut visited = vec![false; (x + y) as usize];

        let mut queue: VecDeque<i32> = VecDeque::new();
        queue.push_back(0);
        while let Some(front) = queue.pop_front() {
            for diff in [x, y, -x, -y].iter() {
                let next = front + diff;
                if next == z {
                    return true;
                } else {
                    if next < x + y && next > 0 && !visited[next as usize] {
                        visited[next as usize] = true;
                        queue.push_back(next);
                    }
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let x = 3;
    let y = 5;
    let z = 4;
    let res = true;
    assert_eq!(Solution::can_measure_water(x, y, z), res);
    let x = 2;
    let y = 6;
    let z = 5;
    let res = false;
    assert_eq!(Solution::can_measure_water(x, y, z), res);
}
