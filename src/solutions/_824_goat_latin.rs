struct Solution;

#[allow(clippy::wrong_self_convention)]
impl Solution {
    fn to_goat_latin(s: String) -> String {
        let words: Vec<&str> = s.split_whitespace().map(|s| s.chars().as_str()).collect();
        let mut res: String = "".to_string();
        let mut n = 1;
        for word in words {
            if n > 1 {
                res += " ";
            }
            match &word[0..1] {
                "a" | "e" | "i" | "o" | "u" | "A" | "E" | "I" | "O" | "U" => {
                    res += word;
                }
                _ => {
                    res += &word[1..];
                    res += &word[0..1];
                }
            }
            res += "ma";
            for _ in 0..n {
                res += "a";
            }
            n += 1;
        }
        res
    }
}

#[test]
fn test() {
    let s = "I speak Goat Latin".to_string();
    let res = "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string();
    assert_eq!(Solution::to_goat_latin(s), res);
    let s = "The quick brown fox jumped over the lazy dog".to_string();
    let res = "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa".to_string();
    assert_eq!(Solution::to_goat_latin(s), res);
}
