struct Solution;

use std::collections::HashSet;

impl Solution {
    fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let m = languages.len();
        let mut people: Vec<HashSet<i32>> = vec![HashSet::new(); m];
        for i in 0..m {
            for &j in languages[i].iter() {
                people[i].insert(j);
            }
        }
        let friendships: Vec<Vec<i32>> = friendships
            .into_iter()
            .filter(|rel| {
                let u = rel[0] as usize - 1;
                let v = rel[1] as usize - 1;
                people[u].is_disjoint(&people[v])
            })
            .collect();
        let mut res = m;
        let mut disjoints = vec![vec![false; m]; m];
        for pair in friendships.iter() {
            let u = pair[0] as usize - 1;
            let v = pair[1] as usize - 1;
            if people[u].is_disjoint(&people[v]) {
                disjoints[u][v] = true;
            }
        }

        for i in 1..=n {
            let mut teach = HashSet::new();
            for pair in friendships.iter() {
                let u = pair[0] as usize - 1;
                let v = pair[1] as usize - 1;
                if disjoints[u][v] {
                    if !people[u].contains(&i) {
                        teach.insert(u);
                    }
                    if !people[v].contains(&i) {
                        teach.insert(v);
                    }
                }
            }
            res = res.min(teach.len());
        }
        res as i32
    }
}

#[test]
fn test() {
    let n = 2;
    let languages = vec_vec_i32![[1], [2], [1, 2]];
    let friendships = vec_vec_i32![[1, 2], [1, 3], [2, 3]];
    let res = 1;
    assert_eq!(Solution::minimum_teachings(n, languages, friendships), res);
    let n = 3;
    let languages = vec_vec_i32![[2], [1, 3], [1, 2], [3]];
    let friendships = vec_vec_i32![[1, 4], [1, 2], [3, 4], [2, 3]];
    let res = 2;
    assert_eq!(Solution::minimum_teachings(n, languages, friendships), res);
}
