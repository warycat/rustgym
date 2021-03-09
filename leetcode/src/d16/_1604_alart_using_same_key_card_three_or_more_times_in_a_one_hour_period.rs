struct Solution;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let n = key_name.len();
        let mut alerted: HashSet<String> = HashSet::new();
        let mut queues: HashMap<String, BTreeSet<i32>> = HashMap::new();
        for i in 0..n {
            if alerted.contains(&key_name[i]) {
                continue;
            }
            let times = queues.entry(key_name[i].to_string()).or_default();
            let t = Self::parse_time(&key_time[i]);
            times.insert(t);
            let sorted_times: Vec<i32> = times.iter().copied().collect();
            if sorted_times.windows(3).any(|w| w[0] + 60 >= w[2]) {
                alerted.insert(key_name[i].to_string());
            }
        }
        let mut res: Vec<String> = alerted.into_iter().collect();
        res.sort_unstable();
        res
    }
    fn parse_time(s: &str) -> i32 {
        s[..2].parse::<i32>().unwrap() * 60 + s[3..].parse::<i32>().unwrap()
    }
}

#[test]
fn test() {
    let key_name = vec_string!["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"];
    let key_time = vec_string!["10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00"];
    let res = vec_string!["daniel"];
    assert_eq!(Solution::alert_names(key_name, key_time), res);
    let key_name = vec_string!["alice", "alice", "alice", "bob", "bob", "bob", "bob"];
    let key_time = vec_string!["12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00"];
    let res = vec_string!["bob"];
    assert_eq!(Solution::alert_names(key_name, key_time), res);
    let key_name = vec_string!["john", "john", "john"];
    let key_time = vec_string!["23:58", "23:59", "00:01"];
    let res = vec_string![];
    assert_eq!(Solution::alert_names(key_name, key_time), res);
    let key_name = vec_string!["leslie", "leslie", "leslie", "clare", "clare", "clare", "clare"];
    let key_time = vec_string!["13:00", "13:20", "14:00", "18:00", "18:51", "19:30", "19:49"];
    let res = vec_string!["clare", "leslie"];
    assert_eq!(Solution::alert_names(key_name, key_time), res);
}
