struct Solution;

impl Solution {
    fn is_decomposable(s: String) -> bool {
        let mut count = 0;
        let mut prev = ' ';
        let mut two = false;
        for c in s.chars() {
            dbg!(c);
            dbg!(count);
            if c == prev {
                count += 1;
            } else {
                if count % 3 == 1 {
                    return false;
                }
                if count % 3 == 2 {
                    if two {
                        return false;
                    } else {
                        two = true;
                    }
                }
                count = 1;
                prev = c;
            }
        }
        if count % 3 == 1 {
            return false;
        }
        if count % 3 == 2 {
            if two {
                return false;
            } else {
                two = true;
            }
        }
        two
    }
}

#[test]
fn test() {
    let s = "000111000".to_string();
    let res = false;
    assert_eq!(Solution::is_decomposable(s), res);
    let s = "00011111222".to_string();
    let res = true;
    assert_eq!(Solution::is_decomposable(s), res);
    let s = "011100022233".to_string();
    let res = false;
    assert_eq!(Solution::is_decomposable(s), res);
}
