struct Solution;

impl Solution {
    fn valid_utf8(data: Vec<i32>) -> bool {
        let mut count = 0;
        for x in data {
            if count == 0 {
                if x >> 3 & 0b11111 == 0b11110 {
                    count += 3;
                    continue;
                }
                if x >> 4 & 0b1111 == 0b1110 {
                    count += 2;
                    continue;
                }
                if x >> 5 & 0b111 == 0b110 {
                    count += 1;
                    continue;
                }
                if x >> 7 & 0b1 == 0 {
                    continue;
                }
            } else {
                if x >> 6 & 0b11 == 0b10 {
                    count -= 1;
                    continue;
                }
            }
            return false;
        }
        count == 0
    }
}

#[test]
fn test() {
    let data = vec![197, 130, 1];
    assert_eq!(Solution::valid_utf8(data), true);
    let data = vec![235, 140, 4];
    assert_eq!(Solution::valid_utf8(data), false);
}
