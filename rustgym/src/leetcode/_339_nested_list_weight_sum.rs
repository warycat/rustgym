struct Solution;
use rustgym_util::*;

impl Solution {
    fn depth_sum_r(nested_list: &NestedInteger, level: i32) -> i32 {
        match nested_list {
            NestedInteger::Int(x) => level * x,
            NestedInteger::List(v) => v
                .iter()
                .fold(0, |acc, x| acc + Self::depth_sum_r(x, level + 1)),
        }
    }
    fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
        nested_list
            .iter()
            .fold(0, |acc, x| acc + Self::depth_sum_r(x, 1))
    }
}

#[test]
fn test() {
    let list = vec![
        nested_integer!("[1,1]"),
        nested_integer!("2"),
        nested_integer!("[1,1]"),
    ];
    assert_eq!(Solution::depth_sum(list), 10);
    let list = vec![nested_integer!("1"), nested_integer!("[4,[6]]")];
    assert_eq!(Solution::depth_sum(list), 27);
}
