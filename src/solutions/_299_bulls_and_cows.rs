struct Solution;

impl Solution {
    fn get_hint(secret: String, guess: String) -> String {
        let s: Vec<char> = secret.chars().collect();
        let g: Vec<char> = guess.chars().collect();
        let mut bulls = 0;
        let mut cows = 0;
        let mut s_count = vec![0; 10];
        let mut g_count = vec![0; 10];
        let n = s.len();
        let m = g.len();
        for i in 0..n.max(m) {
            if i < n.min(m) && s[i] == g[i] {
                bulls += 1;
            } else {
                if i < n {
                    s_count[(s[i] as u8 - b'0') as usize] += 1;
                }
                if i < m {
                    g_count[(g[i] as u8 - b'0') as usize] += 1;
                }
            }
        }
        for i in 0..10 {
            cows += s_count[i].min(g_count[i]);
        }
        format!("{}A{}B", bulls, cows)
    }
}

#[test]
fn test() {
    let secret = "1807".to_string();
    let guess = "7810".to_string();
    let res = "1A3B".to_string();
    assert_eq!(Solution::get_hint(secret, guess), res);
    let secret = "1123".to_string();
    let guess = "0111".to_string();
    let res = "1A1B".to_string();
    assert_eq!(Solution::get_hint(secret, guess), res);
}
