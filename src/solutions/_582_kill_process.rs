struct Solution;
use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    fn kill_process(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut hm: HashMap<i32, Vec<i32>> = HashMap::new();
        let n = pid.len();
        for i in 0..n {
            let x = pid[i];
            let y = ppid[i];
            hm.entry(y).or_default().push(x);
        }
        queue.push_back(kill);
        while let Some(front) = queue.pop_front() {
            res.push(front);
            if let Some(children) = hm.get(&front) {
                for &child in children {
                    queue.push_back(child);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let pid = vec![1, 3, 10, 5];
    let ppid = vec![3, 0, 5, 3];
    let kill = 5;
    let res = vec![5, 10];
    assert_eq!(Solution::kill_process(pid, ppid, kill), res);
}
