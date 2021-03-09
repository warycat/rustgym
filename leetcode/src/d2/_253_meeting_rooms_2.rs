struct Solution;

use std::cmp::Ordering;

impl Solution {
    fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut v: Vec<(i32, i32)> = vec![];
        for i in intervals {
            v.push((i[0], 1));
            v.push((i[1], -1));
        }
        v.sort_unstable_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Equal => a.1.cmp(&b.1),
            x => x,
        });
        let mut rooms = 0;
        let mut max = 0;
        for x in v {
            rooms += x.1;
            max = i32::max(rooms, max);
        }
        max
    }
}

#[test]
fn test() {
    let intervals: Vec<Vec<i32>> = vec_vec_i32![[0, 30], [5, 10], [15, 20]];
    assert_eq!(Solution::min_meeting_rooms(intervals), 2);
    let intervals: Vec<Vec<i32>> = vec_vec_i32![[7, 10], [2, 4]];
    assert_eq!(Solution::min_meeting_rooms(intervals), 1);
    let intervals: Vec<Vec<i32>> = vec_vec_i32![[13, 15], [1, 13]];
    assert_eq!(Solution::min_meeting_rooms(intervals), 1);
}
