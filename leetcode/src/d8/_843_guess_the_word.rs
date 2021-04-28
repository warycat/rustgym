struct Solution;

use std::cell::Cell;
use std::collections::HashSet;
use std::iter::FromIterator;

use rand::{thread_rng, Rng};

fn exact_matches(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(a, b)| a == b).count()
}

impl Solution {
    fn find_secret_word(words: Vec<String>, master: &Master) {
        let n = words.len();
        let mut matrix: Vec<Vec<usize>> = vec![vec![0; n]; n];
        let mut excluded: Vec<bool> = vec![false; n];
        for i in 0..n {
            for j in i + 1..n {
                let count = exact_matches(&words[i], &words[j]);
                matrix[i][j] = count;
                matrix[j][i] = count;
            }
        }
        let mut rng = thread_rng();

        for _ in 0..10 {
            let mut id = rng.gen_range(0, n);
            while excluded[id] {
                id = rng.gen_range(0, n);
            }
            let count = master.guess(words[id].to_string()) as usize;
            if count == 6 {
                break;
            }
            for i in 0..n {
                if matrix[id][i] < count {
                    excluded[i] = true;
                }
            }
        }
    }
}

struct Master {
    list: HashSet<String>,
    secret: String,
    call_count: Cell<usize>,
    guessed: Cell<bool>,
}

impl Master {
    fn new(secret: String, words: Vec<String>) -> Self {
        let call_count = Cell::new(0);
        let guessed = Cell::new(false);
        let list: HashSet<String> = HashSet::from_iter(words);
        Master {
            list,
            secret,
            call_count,
            guessed,
        }
    }
    fn guess(&self, word: String) -> i32 {
        self.call_count.set(self.call_count.get() + 1);
        if word == self.secret {
            self.guessed.set(true);
        }
        if self.list.contains(&word) {
            exact_matches(&self.secret, &word) as i32
        } else {
            -1
        }
    }
    fn pass(&self) -> bool {
        self.call_count.get() <= 10 && self.guessed.get()
    }
}

#[test]
fn test() {
    let secret = "acckzz".to_string();
    let wordlist = vec_string!["acckzz", "ccbazz", "eiowzz", "abcczz"];
    let master = Master::new(secret, wordlist.to_vec());
    Solution::find_secret_word(wordlist, &master);
    assert_eq!(master.pass(), true);
}
