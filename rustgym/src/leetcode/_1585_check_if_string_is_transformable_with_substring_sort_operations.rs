struct Solution;

impl Solution {
    fn is_transformable(s: String, t: String) -> bool {
        let n = s.len();
        let s: Vec<u8> = s.bytes().collect();
        let t: Vec<u8> = t.bytes().collect();
        let mut idx: Vec<Vec<usize>> = vec![vec![]; 10];
        for i in (0..n).rev() {
            idx[(s[i] - b'0') as usize].push(i);
        }
        for i in 0..n {
            let k = (t[i] - b'0') as usize;
            if idx[k].is_empty() {
                return false;
            }
            let p = idx[k].pop().unwrap();
            for j in 0..k {
                if let Some(&q) = idx[j].last() {
                    if q < p {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let s = "84532".to_string();
    let t = "34852".to_string();
    let res = true;
    assert_eq!(Solution::is_transformable(s, t), res);
    let s = "34521".to_string();
    let t = "23415".to_string();
    let res = true;
    assert_eq!(Solution::is_transformable(s, t), res);
    let s = "12345".to_string();
    let t = "12435".to_string();
    let res = false;
    assert_eq!(Solution::is_transformable(s, t), res);
    let s = "1".to_string();
    let t = "2".to_string();
    let res = false;
    assert_eq!(Solution::is_transformable(s, t), res);
}
