struct Solution;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
}

impl Solution {
    fn count_distinct(s: String) -> i32 {
        let n = s.len();
        let s: Vec<u8> = s.bytes().collect();
        let mut root: Trie = Trie::default();
        let mut res = 0;
        for i in 0..n {
            let mut cur = &mut root;
            for j in i..n {
                if cur.children[(s[j] - b'a') as usize].is_none() {
                    cur.children[(s[j] - b'a') as usize] = Some(Box::new(Trie::default()));
                    res += 1;
                }
                cur = cur.children[(s[j] - b'a') as usize].as_mut().unwrap();
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "aabbaba".to_string();
    let res = 21;
    assert_eq!(Solution::count_distinct(s), res);
    let s = "abcdefg".to_string();
    let res = 28;
    assert_eq!(Solution::count_distinct(s), res);
}
