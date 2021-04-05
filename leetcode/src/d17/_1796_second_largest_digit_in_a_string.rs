struct Solution;

impl Solution {
    fn second_highest(s: String) -> i32 {
        let mut set: Vec<bool> = vec![false; 10];
        for b in s.bytes() {
            if (b'0'..=b'9').contains(&b) {
                let i = (b - b'0') as usize;
                set[i] = true;
            }
        }
        let mut largest: Vec<i32> = vec![];
        for i in (0..10).rev() {
            if set[i] {
                largest.push(i as i32);
            }
        }
        if largest.len() < 2 {
            -1
        } else {
            largest[1]
        }
    }
}

#[test]
fn test() {
    let s = "dfa12321afd".to_string();
    let res = 2;
    assert_eq!(Solution::second_highest(s), res);
    let s = "abc1111".to_string();
    let res = -1;
    assert_eq!(Solution::second_highest(s), res);
}
