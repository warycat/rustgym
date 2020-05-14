struct Solution;
use std::collections::HashMap;
use std::collections::HashSet;

type Sequence = (String, String, String);

impl Solution {
    fn most_visited_pattern(
        mut username: Vec<String>,
        mut timestamp: Vec<i32>,
        mut website: Vec<String>,
    ) -> Vec<String> {
        let mut hm: HashMap<String, Vec<(i32, String)>> = HashMap::new();
        let mut sequences: HashMap<Sequence, HashSet<String>> = HashMap::new();
        while let (Some(u), Some(t), Some(w)) = (username.pop(), timestamp.pop(), website.pop()) {
            hm.entry(u).or_default().push((t, w));
        }
        for (u, p) in hm.iter_mut() {
            p.sort_unstable_by_key(|p| p.0);
            let n = p.len();
            for i in 0..n {
                for j in i + 1..n {
                    for k in j + 1..n {
                        let sequence = (p[i].1.to_string(), p[j].1.to_string(), p[k].1.to_string());
                        sequences.entry(sequence).or_default().insert(u.to_string());
                    }
                }
            }
        }
        let mut max = 0;
        let mut res = ("".to_string(), "".to_string(), "".to_string());
        for (sequence, hs) in sequences {
            if hs.len() > max {
                max = hs.len();
                res = sequence;
            } else if hs.len() == max && sequence < res {
                res = sequence;
            }
        }
        vec![res.0, res.1, res.2]
    }
}

#[test]
fn test() {
    let username = vec_string![
        "joe", "joe", "joe", "james", "james", "james", "james", "mary", "mary", "mary"
    ];
    let timestamp = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let website = vec_string![
        "home", "about", "career", "home", "cart", "maps", "home", "home", "about", "career"
    ];
    let res = vec_string!["home", "about", "career"];
    assert_eq!(
        Solution::most_visited_pattern(username, timestamp, website),
        res
    );
}
