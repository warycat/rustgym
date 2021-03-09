struct Solution;

struct Word {
    data: Vec<(char, usize)>,
}

impl Word {
    fn new(s: String) -> Self {
        let mut data = vec![];
        let mut prev: Option<(char, usize)> = None;
        for c in s.chars() {
            if let Some(p) = prev {
                if c == p.0 {
                    prev = Some((c, p.1 + 1));
                } else {
                    data.push(p);
                    prev = Some((c, 1));
                }
            } else {
                prev = Some((c, 1));
            }
        }
        if let Some(p) = prev {
            data.push(p);
        }
        Word { data }
    }

    fn stretchy(&self, word: &Word) -> bool {
        let n = self.data.len();
        let m = word.data.len();
        if n != m {
            return false;
        }
        for i in 0..n {
            let p = self.data[i];
            let q = word.data[i];
            if p.0 != q.0 || p.1 > q.1 || (q.1 < 3 && p.1 != q.1) {
                return false;
            }
        }
        true
    }
}

impl Solution {
    fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let word = Word::new(s);
        let words: Vec<Word> = words.into_iter().map(Word::new).collect();
        words.iter().filter(|w| w.stretchy(&word)).count() as i32
    }
}

#[test]
fn test() {
    let s = "heeellooo".to_string();
    let words = vec_string!["hello", "hi", "helo"];
    let res = 1;
    assert_eq!(Solution::expressive_words(s, words), res);
}
