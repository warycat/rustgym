struct Solution;

impl Solution {
    fn compress(chars: &mut Vec<char>) -> i32 {
        let mut j: usize = 0;
        let mut prev: Option<char> = None;
        let mut count = 0;
        let n = chars.len();
        for i in 0..n {
            if let Some(c) = prev {
                if c == chars[i] {
                    count += 1;
                } else {
                    j += Self::write_pair(chars, j, c, count);
                    prev = Some(chars[i]);
                    count = 1;
                }
            } else {
                count = 1;
                prev = Some(chars[i]);
            }
        }
        if let Some(c) = prev {
            j += Self::write_pair(chars, j, c, count);
        }
        j as i32
    }

    fn write_pair(chars: &mut Vec<char>, mut index: usize, c: char, mut count: usize) -> usize {
        chars[index] = c;
        index += 1;
        if count == 1 {
            return 1;
        }
        let mut size: usize = 0;
        while count > 0 {
            let d: u8 = count as u8 % 10u8 + b'0';
            chars[index + size] = d as char;
            size += 1;
            count /= 10;
        }
        let mut i = index;
        let mut j = index + size - 1;
        while i < j {
            chars.swap(i, j);
            i += 1;
            j -= 1;
        }
        1 + size
    }
}

#[test]
fn test() {
    let mut input: Vec<char> = [
        "a", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b",
    ]
    .iter()
    .map(|s| s.chars().next().unwrap())
    .collect();
    assert_eq!(Solution::compress(&mut input), 4);
}
