struct Solution;

impl Solution {
    fn reverse_only_letters(s: String) -> String {
        let n = s.len();
        if n == 0 {
            return "".to_string();
        }
        let mut a: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = n - 1;
        while i < j {
            while i < j && !a[i].is_alphabetic() {
                i += 1;
            }
            while i < j && !a[j].is_alphabetic() {
                j -= 1;
            }
            if i < j {
                a.swap(i, j);
            }
            i += 1;
            j -= 1;
        }
        a.iter().collect()
    }
}

#[test]
fn test() {
    let s = "ab-cd".to_string();
    let t = "dc-ba".to_string();
    assert_eq!(Solution::reverse_only_letters(s), t);
    let s = "a-bC-dEf-ghIj".to_string();
    let t = "j-Ih-gfE-dCba".to_string();
    assert_eq!(Solution::reverse_only_letters(s), t);
    let s = "Test1ng-Leet=code-Q!".to_string();
    let t = "Qedo1ct-eeLg=ntse-T!".to_string();
    assert_eq!(Solution::reverse_only_letters(s), t);
}
