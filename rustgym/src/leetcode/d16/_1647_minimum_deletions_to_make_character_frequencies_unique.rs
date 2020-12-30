struct Solution;

impl Solution {
    fn min_deletions(s: String) -> i32 {
        let mut count = vec![0; 26];
        for b in s.bytes() {
            count[(b - b'a') as usize] += 1;
        }
        let mut res = 0;
        for i in 0..26 {
            'outer: while count[i] != 0 {
                for j in 0..26 {
                    if j != i && count[i] == count[j] {
                        count[i] -= 1;
                        res += 1;
                        continue 'outer;
                    }
                }
                break;
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "aab".to_string();
    let res = 0;
    assert_eq!(Solution::min_deletions(s), res);
    let s = "aaabbbcc".to_string();
    let res = 2;
    assert_eq!(Solution::min_deletions(s), res);
    let s = "ceabaacb".to_string();
    let res = 2;
    assert_eq!(Solution::min_deletions(s), res);
}
