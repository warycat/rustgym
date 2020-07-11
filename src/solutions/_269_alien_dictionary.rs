struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn alien_order(words: Vec<String>) -> String {
        let mut alphabet = vec![false; 256];
        for s in words.iter() {
            for b in s.chars() {
                alphabet[b as usize] = true;
            }
        }
        let k: usize = alphabet.iter().map(|&b| if b { 1 } else { 0 }).sum();
        let mut edges: Vec<Vec<u8>> = vec![vec![]; 256];
        for w in words.windows(2) {
            if w[0] == w[1] {
                continue;
            }
            if w[0].starts_with(&w[1]) {
                return "".to_string();
            }
            if let Some((t, h)) = w[0]
                .bytes()
                .zip(w[1].bytes())
                .skip_while(|(t, h)| t == h)
                .take(1)
                .next()
            {
                edges[t as usize].push(h);
            }
        }
        let mut indegree: Vec<usize> = vec![0; 256];
        for i in 0..256 {
            for &h in &edges[i] {
                indegree[h as usize] += 1;
            }
        }
        let mut queue: VecDeque<u8> = VecDeque::new();
        for i in 0..256 {
            if alphabet[i] && indegree[i] == 0 {
                queue.push_back(i as u8);
            }
        }

        let mut res = "".to_string();
        while let Some(t) = queue.pop_front() {
            res.push(t as char);
            let n = edges[t as usize].len();
            for i in 0..n {
                let h = edges[t as usize][i];
                indegree[h as usize] -= 1;
                if indegree[h as usize] == 0 {
                    queue.push_back(h);
                }
            }
        }
        if k == res.len() {
            res
        } else {
            "".to_string()
        }
    }
}

#[test]
fn test() {
    let words = vec_string!["wrt", "wrf", "er", "ett", "rftt"];
    let res = "wertf".to_string();
    assert_eq!(Solution::alien_order(words), res);
    let words = vec_string!["z", "x"];
    let res = "zx".to_string();
    assert_eq!(Solution::alien_order(words), res);
    let words = vec_string!["z", "x", "z"];
    let res = "".to_string();
    assert_eq!(Solution::alien_order(words), res);
    let words = vec_string!["abc", "ab"];
    let res = "".to_string();
    assert_eq!(Solution::alien_order(words), res);
}
