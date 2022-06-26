struct Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut if_count = true;
        let mut count = 0;

        s.chars().for_each(|c| match c {
            '|' => if_count = !if_count,
            '*' => {
                if if_count {
                    count += 1;
                }
            }
            _ => {}
        });

        count
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::count_asterisks("l|*e*et|c**o|*de|".to_string()),
        2
    );
    assert_eq!(Solution::count_asterisks("iamprogrammer".to_string()), 0);
    assert_eq!(
        Solution::count_asterisks("yo|uar|e**|b|e***au|tifu|l".to_string()),
        5
    );
}
