struct Solution;
use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    fn watched_videos_by_friends(
        watched_videos: Vec<Vec<String>>,
        friends: Vec<Vec<i32>>,
        id: i32,
        level: i32,
    ) -> Vec<String> {
        let n = watched_videos.len();
        let mut visited = vec![false; n];
        let mut queue: VecDeque<(usize, i32)> = VecDeque::new();
        let id = id as usize;
        visited[id] = true;
        queue.push_back((id, 0));
        let mut freq: HashMap<String, usize> = HashMap::new();
        while let Some((u, l)) = queue.pop_front() {
            if l < level {
                for &friend in &friends[u] {
                    let v = friend as usize;
                    if !visited[v] {
                        visited[v] = true;
                        queue.push_back((v, l + 1));
                    }
                }
            } else {
                for video in &watched_videos[u] {
                    *freq.entry(video.to_string()).or_default() += 1;
                }
            }
        }
        let mut pairs: Vec<(usize, String)> = vec![];
        for (video, count) in freq {
            pairs.push((count, video));
        }
        pairs.sort_unstable();
        pairs.into_iter().map(|p| p.1).collect()
    }
}

#[test]
fn test() {
    let watched_videos = vec_vec_string![["A", "B"], ["C"], ["B", "C"], ["D"]];
    let friends = vec_vec_i32![[1, 2], [0, 3], [0, 3], [1, 2]];
    let id = 0;
    let level = 1;
    let res = vec_string!["B", "C"];
    assert_eq!(
        Solution::watched_videos_by_friends(watched_videos, friends, id, level),
        res
    );
    let watched_videos = vec_vec_string![["A", "B"], ["C"], ["B", "C"], ["D"]];
    let friends = vec_vec_i32![[1, 2], [0, 3], [0, 3], [1, 2]];
    let id = 0;
    let level = 2;
    let res = vec_string!["D"];
    assert_eq!(
        Solution::watched_videos_by_friends(watched_videos, friends, id, level),
        res
    );
}
