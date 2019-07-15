struct Solution {}

enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

impl Solution {
    fn depth_sum_r(nested_list: &NestedInteger, level: i32) -> i32 {
        match nested_list {
            NestedInteger::Int(x) => level * x,
            NestedInteger::List(v) => v
                .iter()
                .fold(0, |acc, x| acc + Solution::depth_sum_r(x, level + 1)),
        }
    }
    fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
        nested_list
            .iter()
            .fold(0, |acc, x| acc + Solution::depth_sum_r(x, 1))
    }
}

#[test]
fn test() {
    let list = vec![
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        NestedInteger::Int(2),
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
    ];
    assert_eq!(Solution::depth_sum(list), 10);
    let list = vec![
        NestedInteger::Int(1),
        NestedInteger::List(vec![
            NestedInteger::Int(4),
            NestedInteger::List(vec![NestedInteger::Int(6)]),
        ]),
    ];
    assert_eq!(Solution::depth_sum(list), 27);
}
