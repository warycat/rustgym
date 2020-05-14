struct Solution;

impl Solution {
    fn valid_word_square(words: Vec<String>) -> bool {
        let n = words.len();
        for i in 0..n {
            let m = words[i].len();
            if m > n {
                return false;
            }
        }
        let mut a: Vec<Vec<u8>> = vec![vec![0; n]; n];
        for i in 0..n {
            for (j, b) in words[i].bytes().enumerate() {
                a[i][j] = b;
            }
        }
        for i in 0..n {
            for j in 0..n {
                if a[i][j] != a[j][i] {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let words: Vec<String> = vec_string!["abcd", "bnrt", "crmy", "dtye"];
    assert_eq!(Solution::valid_word_square(words), true);
    let words: Vec<String> = vec_string!["abcd", "bnrt", "crm", "dt"];
    assert_eq!(Solution::valid_word_square(words), true);
    let words: Vec<String> = vec_string!["ball", "area", "read", "lady"];
    assert_eq!(Solution::valid_word_square(words), false);
    let words: Vec<String> = vec_string!["ball", "asee", "let", "lep"];
    assert_eq!(Solution::valid_word_square(words), false);
}
