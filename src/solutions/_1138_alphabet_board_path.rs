struct Solution;

impl Solution {
    fn alphabet_board_path(target: String) -> String {
        let mut pos: Vec<(i32, i32)> = vec![];
        for i in 0..5 {
            for j in 0..5 {
                pos.push((i, j));
            }
        }
        pos.push((5, 0));

        let mut v = vec!['a'];
        for c in target.chars() {
            v.push(c);
        }
        let n = v.len();
        let mut res = "".to_string();
        for i in 1..n {
            let curr = pos[(v[i] as u8 - b'a') as usize];
            let prev = pos[(v[i - 1] as u8 - b'a') as usize];
            let mut r = curr.0 - prev.0;
            let mut c = curr.1 - prev.1;
            while r < 0 {
                res.push('U');
                r += 1;
            }
            while c < 0 {
                res.push('L');
                c += 1;
            }
            while r > 0 {
                res.push('D');
                r -= 1;
            }
            while c > 0 {
                res.push('R');
                c -= 1;
            }
            res.push('!');
        }
        res
    }
}

#[test]
fn test() {
    let target = "leet".to_string();
    let res = "DDR!UURRR!!DDD!".to_string();
    assert_eq!(Solution::alphabet_board_path(target), res);
    let target = "code".to_string();
    let res = "RR!DDRR!UUL!R!".to_string();
    assert_eq!(Solution::alphabet_board_path(target), res);
}
