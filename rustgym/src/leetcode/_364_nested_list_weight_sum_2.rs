struct Solution;
use rustgym_util::*;

impl Solution {
    fn depth_sum_inverse(nested_list: Vec<NestedInteger>) -> i32 {
        use NestedInteger::*;
        let mut weighted = 0;
        let mut unweighted = 0;
        let mut v: Vec<NestedInteger> = nested_list;
        while !v.is_empty() {
            let mut u: Vec<NestedInteger> = vec![];
            for n in v.into_iter() {
                match n {
                    Int(x) => unweighted += x,
                    List(v) => {
                        for x in v.into_iter() {
                            u.push(x);
                        }
                    }
                }
            }
            weighted += unweighted;
            v = u;
        }
        weighted
    }
}

#[test]
fn test() {
    let nested_list = nested_integer!("[[1,1],2,[1,1]]");
    assert_eq!(Solution::depth_sum_inverse(vec![nested_list]), 8);
}
