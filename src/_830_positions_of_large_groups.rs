struct Solution;

struct Group {
    c: char,
    start: usize,
    end: usize,
}

impl Solution {
    fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut prev: Option<Group> = None;
        let mut groups: Vec<Group> = vec![];
        for (i, c) in s.chars().enumerate() {
            if let Some(prev_group) = prev {
                if prev_group.c == c {
                    prev = Some(Group {
                        c,
                        start: prev_group.start,
                        end: i,
                    });
                } else {
                    groups.push(prev_group);
                    prev = Some(Group {
                        c,
                        start: i,
                        end: i,
                    });
                }
            } else {
                prev = Some(Group {
                    c,
                    start: i,
                    end: i,
                });
            }
        }
        if let Some(prev_group) = prev {
            groups.push(prev_group);
        }

        groups
            .iter()
            .filter_map(|g| {
                let start = g.start;
                let end = g.end;
                if end - start > 1 {
                    Some(vec![start as i32, end as i32])
                } else {
                    None
                }
            })
            .collect()
    }
}

#[test]
fn test() {
    let s = "abbxxxxzzy".to_string();
    let res: Vec<Vec<i32>> = vec_vec_i32![[3, 6]];
    assert_eq!(Solution::large_group_positions(s), res);
    let s = "abc".to_string();
    let res: Vec<Vec<i32>> = vec![];
    assert_eq!(Solution::large_group_positions(s), res);
    let s = "abcdddeeeeaabbbcd".to_string();
    let res: Vec<Vec<i32>> = vec_vec_i32![[3, 5], [6, 9], [12, 14]];
    assert_eq!(Solution::large_group_positions(s), res);
}
