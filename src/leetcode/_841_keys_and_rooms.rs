struct Solution;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut visited: HashSet<usize> = HashSet::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(0);
        while let Some(front) = queue.pop_front() {
            let room = &rooms[front];
            if visited.insert(front) {
                for &key in room {
                    queue.push_back(key as usize);
                }
            }
        }
        visited.len() == n
    }
}

#[test]
fn test() {
    let rooms = vec_vec_i32![[1], [2], [3], []];
    let res = true;
    assert_eq!(Solution::can_visit_all_rooms(rooms), res);
    let rooms = vec_vec_i32![[1, 3], [3, 0, 1], [2], [0]];
    let res = false;
    assert_eq!(Solution::can_visit_all_rooms(rooms), res);
}
