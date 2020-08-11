struct Solution;
use rustgym_util::*;

impl Solution {
    fn deserialize(s: String) -> NestedInteger {
        nested_integer!(s)
    }
}

#[test]
fn test() {
    let s = "324".to_string();
    let res = nested_integer!("324");
    assert_eq!(Solution::deserialize(s), res);
    let s = "[123,[456,[789]]]".to_string();
    let res = nested_integer!("[123,[456,[789]]]");
    assert_eq!(Solution::deserialize(s), res);
    let s = "[123,456,[788,799,833],[[]],10,[]]".to_string();
    let res = nested_integer!("[123,456,[788,799,833],[[]],10,[]]");
    assert_eq!(Solution::deserialize(s), res);
}
