struct Solution;

impl Solution {
    fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut queues: Vec<Vec<_>> = vec![vec![]; 26];
        let mut temp: Vec<(char, _)> = vec![];
        for word in &words {
            let mut iter = word.chars();
            if let Some(c) = iter.next() {
                queues[(c as u8 - b'a') as usize].push(iter);
            }
        }
        let mut res = 0;
        for c in s.chars() {
            let iters = &mut queues[(c as u8 - b'a') as usize];
            while let Some(mut iter) = iters.pop() {
                if let Some(d) = iter.next() {
                    temp.push((d, iter));
                } else {
                    res += 1;
                }
            }
            while let Some((c, iter)) = temp.pop() {
                queues[(c as u8 - b'a') as usize].push(iter);
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "abcde".to_string();
    let words = vec_string!["a", "bb", "acd", "ace"];
    let res = 3;
    assert_eq!(Solution::num_matching_subseq(s, words), res);
}
