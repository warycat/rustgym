struct Solution;

impl Solution {
    fn words_typing(sentence: Vec<String>, rows: i32, cols: i32) -> i32 {
        let size: Vec<usize> = sentence.into_iter().map(|s| s.len()).collect();
        let n = size.len();
        let sum: usize = size.iter().sum::<usize>() + n;
        let mut i = 0;
        let rows = rows as usize;
        let cols = cols as usize;
        let mut res = 0;
        for _ in 0..rows {
            if size[i] <= cols {
                let m = cols / sum;
                res += m;
                let mut start = m * sum;
                if start + size[i] <= cols {
                    start += size[i];
                    i += 1;
                    if i == n {
                        i = 0;
                        res += 1;
                    }
                } else {
                    continue;
                }
                while start + size[i] < cols {
                    start += size[i] + 1;
                    i += 1;
                    if i == n {
                        i = 0;
                        res += 1;
                    }
                }
            } else {
                break;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let rows = 2;
    let cols = 8;
    let sentence = vec_string!["hello", "world"];
    let res = 1;
    assert_eq!(Solution::words_typing(sentence, rows, cols), res);
    let rows = 3;
    let cols = 6;
    let sentence = vec_string!["a", "bcd", "e"];
    let res = 2;
    assert_eq!(Solution::words_typing(sentence, rows, cols), res);
    let rows = 4;
    let cols = 5;
    let sentence = vec_string!["I", "had", "apple", "pie"];
    let res = 1;
    assert_eq!(Solution::words_typing(sentence, rows, cols), res);
}
