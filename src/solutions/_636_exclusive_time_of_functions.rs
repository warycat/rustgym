struct Solution;

enum Action {
    Start,
    End,
}

impl Solution {
    fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let n = n as usize;
        let mut stack: Vec<usize> = vec![];
        let mut res: Vec<i32> = vec![0; n];
        let mut prev = 0;
        for log in logs {
            let mut it = log.split(':');
            let id: usize = it.next().unwrap().parse::<usize>().unwrap();
            let action: Action = if it.next().unwrap() == "start" {
                Action::Start
            } else {
                Action::End
            };
            let val: i32 = it.next().unwrap().parse::<i32>().unwrap();
            match action {
                Action::Start => {
                    if let Some(&top) = stack.last() {
                        res[top] += val - prev;
                    }
                    prev = val;
                    stack.push(id);
                }
                Action::End => {
                    if let Some(top) = stack.pop() {
                        res[top] += val + 1 - prev;
                        prev = val + 1;
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let n = 2;
    let logs = vec_string!["0:start:0", "1:start:2", "1:end:5", "0:end:6"];
    let res = vec![3, 4];
    assert_eq!(Solution::exclusive_time(n, logs), res);
}
