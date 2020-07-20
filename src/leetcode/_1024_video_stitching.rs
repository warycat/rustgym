struct Solution;

use std::cmp::Reverse;

impl Solution {
    fn video_stitching(mut clips: Vec<Vec<i32>>, t: i32) -> i32 {
        clips.sort_by_key(|v| (v[0], Reverse(v[1])));
        let n = clips.len();
        let mut res = 0;
        let mut left = 0;
        let mut right = 0;
        for i in 0..n {
            if clips[i][0] >= t {
                break;
            }
            if clips[i][0] < left {
                right = right.max(clips[i][1]);
            } else if clips[i][0] <= right {
                right = right.max(clips[i][1]);
                left = right;
                res += 1;
            } else {
                return -1;
            }
        }
        if right < t {
            -1
        } else {
            if left < t {
                res + 1
            } else {
                res
            }
        }
    }
}

#[test]
fn test() {
    let clips = vec_vec_i32![[0, 2], [4, 6], [8, 10], [1, 9], [1, 5], [5, 9]];
    let t = 10;
    let res = 3;
    assert_eq!(Solution::video_stitching(clips, t), res);
    let clips = vec_vec_i32![[0, 1], [1, 2]];
    let t = 5;
    let res = -1;
    assert_eq!(Solution::video_stitching(clips, t), res);
    let clips = vec_vec_i32![
        [0, 1],
        [6, 8],
        [0, 2],
        [5, 6],
        [0, 4],
        [0, 3],
        [6, 7],
        [1, 3],
        [4, 7],
        [1, 4],
        [2, 5],
        [2, 6],
        [3, 4],
        [4, 5],
        [5, 7],
        [6, 9]
    ];
    let t = 9;
    let res = 3;
    assert_eq!(Solution::video_stitching(clips, t), res);
    let clips = vec_vec_i32![[0, 4], [2, 8]];
    let t = 5;
    let res = 2;
    assert_eq!(Solution::video_stitching(clips, t), res);
    let clips = vec_vec_i32![
        [5, 7],
        [1, 8],
        [0, 0],
        [2, 3],
        [4, 5],
        [0, 6],
        [5, 10],
        [7, 10]
    ];
    let t = 5;
    let res = 1;
    assert_eq!(Solution::video_stitching(clips, t), res);
}
