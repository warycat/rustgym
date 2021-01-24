struct Solution;

impl Solution {
    fn min_characters(a: String, b: String) -> i32 {
        let a: Vec<u8> = a.bytes().collect();
        let b: Vec<u8> = b.bytes().collect();
        let count_a = Self::check(&a);
        let count_b = Self::check(&b);
        let mut prefix_a = vec![0; 26];
        let mut prev_a = 0;
        let mut prefix_b = vec![0; 26];
        let mut prev_b = 0;
        for i in 0..26 {
            prefix_a[i] = prev_a;
            prev_a += count_a[i];
            prefix_b[i] = prev_b;
            prev_b += count_b[i];
        }
        let mut postfix_a = vec![0; 26];
        let mut next_a = 0;
        let mut postfix_b = vec![0; 26];
        let mut next_b = 0;
        for i in (0..26).rev() {
            postfix_a[i] = next_a;
            next_a += count_a[i];
            postfix_b[i] = next_b;
            next_b += count_b[i];
        }
        let mut max = 0;
        let mut max_a = 0;
        let mut max_b = 0;
        for i in 0..26 {
            max_a = max_a.max(count_a[i]);
            max_b = max_b.max(count_b[i]);
        }
        max = max.max(max_a + max_b);
        for i in 1..26 {
            max = max.max(prefix_a[i] + postfix_b[i - 1]);
            max = max.max(prefix_b[i] + postfix_a[i - 1]);
        }
        (a.len() + b.len() - max) as i32
    }

    fn check(s: &[u8]) -> Vec<usize> {
        let mut count: Vec<usize> = vec![0; 26];
        for b in s {
            count[(b - b'a') as usize] += 1;
        }
        count
    }
}

#[test]
fn test() {
    let a = "aba".to_string();
    let b = "caa".to_string();
    let res = 2;
    assert_eq!(Solution::min_characters(a, b), res);
    let a = "dabadd".to_string();
    let b = "cda".to_string();
    let res = 3;
    assert_eq!(Solution::min_characters(a, b), res);
    let a = "acac".to_string();
    let b = "bd".to_string();
    let res = 1;
    assert_eq!(Solution::min_characters(a, b), res);
}
