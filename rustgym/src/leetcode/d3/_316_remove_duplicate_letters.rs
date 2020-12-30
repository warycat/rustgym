struct Solution;

impl Solution {
    fn remove_duplicate_letters(text: String) -> String {
        let mut stack: Vec<u8> = vec![];
        let mut left: Vec<usize> = vec![0; 26];
        for b in text.bytes() {
            left[(b - b'a') as usize] += 1;
        }
        let mut visited: Vec<bool> = vec![false; 26];
        for b in text.bytes() {
            left[(b - b'a') as usize] -= 1;
            if !visited[(b - b'a') as usize] {
                visited[(b - b'a') as usize] = true;
                while let Some(&top) = stack.last() {
                    if top > b && left[(top - b'a') as usize] > 0 {
                        visited[(top - b'a') as usize] = false;
                        stack.pop();
                    } else {
                        break;
                    }
                }
                stack.push(b);
            }
        }

        stack.into_iter().map(|b| b as char).collect()
    }
}

#[test]
fn test() {
    let text = "cdadabcc".to_string();
    let res = "adbc".to_string();
    assert_eq!(Solution::remove_duplicate_letters(text), res);
    let text = "abcd".to_string();
    let res = "abcd".to_string();
    assert_eq!(Solution::remove_duplicate_letters(text), res);
    let text = "ecbacba".to_string();
    let res = "eacb".to_string();
    assert_eq!(Solution::remove_duplicate_letters(text), res);
    let text = "leetcode".to_string();
    let res = "letcod".to_string();
    assert_eq!(Solution::remove_duplicate_letters(text), res);
}
