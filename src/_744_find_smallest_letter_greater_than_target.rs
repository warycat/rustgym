struct Solution;

impl Solution {
    fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let n = letters.len();
        let mut l = 0;
        let mut r = n;
        while l < r {
            let mid = l + (r - l) / 2;
            if target < letters[mid] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        letters[l % n]
    }
}

#[test]
fn test() {
    let letters: Vec<char> = vec!['c', 'f', 'j'];
    let target = 'a';
    let res = 'c';
    assert_eq!(Solution::next_greatest_letter(letters, target), res);
    let letters: Vec<char> = vec!['c', 'f', 'j'];
    let target = 'c';
    let res = 'f';
    assert_eq!(Solution::next_greatest_letter(letters, target), res);
    let letters: Vec<char> = vec!['c', 'f', 'j'];
    let target = 'd';
    let res = 'f';
    assert_eq!(Solution::next_greatest_letter(letters, target), res);
    let letters: Vec<char> = vec!['c', 'f', 'j'];
    let target = 'g';
    let res = 'j';
    assert_eq!(Solution::next_greatest_letter(letters, target), res);
    let letters: Vec<char> = vec!['c', 'f', 'j'];
    let target = 'j';
    let res = 'c';
    assert_eq!(Solution::next_greatest_letter(letters, target), res);
    let letters: Vec<char> = vec!['c', 'f', 'j'];
    let target = 'k';
    let res = 'c';
    assert_eq!(Solution::next_greatest_letter(letters, target), res);
}
