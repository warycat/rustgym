struct Solution;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Count {
    v: Vec<i32>,
}

impl Count {
    fn new(s: &str) -> Self {
        let mut v: Vec<i32> = vec![0; 256];
        for c in s.chars() {
            v[c as usize] += 1;
        }
        Count { v }
    }

    fn completes(&self, other: &Count) -> bool {
        for i in 0..26 {
            let c: usize = (b'a' + i) as usize;
            if self.v[c] < other.v[c] {
                return false;
            }
        }
        true
    }
}

impl Solution {
    fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut min: Option<String> = None;
        let letters: String = license_plate
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect();
        let lowercase = letters.to_lowercase();
        let count_of_lowercase = Count::new(&lowercase);
        for word in words {
            let count_of_word = Count::new(&word);
            if count_of_word.completes(&count_of_lowercase) {
                if let Some(ref s) = min {
                    if word.len() < s.len() {
                        min = Some(word);
                    }
                } else {
                    min = Some(word);
                }
            }
        }
        min.unwrap()
    }
}

#[test]
fn test() {
    let license_plate = "1s3 PSt".to_string();
    let words: Vec<String> = vec_string!["step", "steps", "stripe", "stepple"];
    let res = "steps".to_string();
    assert_eq!(
        Solution::shortest_completing_word(license_plate, words),
        res
    );
    let license_plate = "1s3 456".to_string();
    let words: Vec<String> = vec_string!["looks", "pest", "stew", "show"];
    let res = "pest".to_string();
    assert_eq!(
        Solution::shortest_completing_word(license_plate, words),
        res
    );
}
