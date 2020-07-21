struct Solution;

impl Solution {
    fn min_abbreviation(target: String, dictionary: Vec<String>) -> String {
        let n = target.len();
        let target: Vec<char> = target.chars().collect();
        let dictionary: Vec<Vec<char>> = dictionary
            .into_iter()
            .filter(|s| s.len() == n)
            .map(|s| s.chars().collect())
            .collect();
        let mut mask: u32 = 0;
        let mut res: u32 = !0;
        Self::dfs(0, &mut mask, &mut res, &target, &dictionary, n);
        Self::abbreviate(&target, res, n)
    }

    fn dfs(
        start: usize,
        mask: &mut u32,
        res: &mut u32,
        target: &[char],
        dictionary: &[Vec<char>],
        n: usize,
    ) {
        if start == n {
            for word in dictionary.iter() {
                if Self::conflict(target, word, *mask, n) {
                    return;
                }
            }
            if Self::length(*mask, n) < Self::length(*res, n) {
                *res = *mask;
            }
        } else {
            Self::dfs(start + 1, mask, res, target, dictionary, n);
            *mask |= 1 << start;
            Self::dfs(start + 1, mask, res, target, dictionary, n);
            *mask &= !(1 << start);
        }
    }

    fn conflict(target: &[char], word: &[char], mask: u32, n: usize) -> bool {
        (0..n).all(|i| mask & 1 << i == 0 || target[i] == word[i])
    }

    fn abbreviate(target: &[char], mask: u32, n: usize) -> String {
        let mut i = 0;
        let mut res = "".to_string();
        while i < n {
            if mask & 1 << i == 0 {
                let mut size = 1;
                while i + 1 < n && mask & 1 << (i + 1) == 0 {
                    size += 1;
                    i += 1;
                }
                res.push_str(&size.to_string());
            } else {
                res.push(target[i]);
            }
            i += 1;
        }
        res
    }

    fn length(mask: u32, n: usize) -> usize {
        let mut i = 0;
        let mut res = 0;
        while i < n {
            if mask & 1 << i == 0 {
                while i + 1 < n && mask & 1 << (i + 1) == 0 {
                    i += 1;
                }
            }
            res += 1;
            i += 1;
        }
        res
    }
}

#[test]
fn test() {
    let target = "apple".to_string();
    let dictionary = vec_string!["blade"];
    let res = "a4".to_string();
    assert_eq!(Solution::min_abbreviation(target, dictionary), res);
    let target = "apple".to_string();
    let dictionary = vec_string!["plain", "amber", "blade"];
    let res = "3l1".to_string();
    assert_eq!(Solution::min_abbreviation(target, dictionary), res);
}
