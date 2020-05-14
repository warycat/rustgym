pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

impl Solution {
    pub fn depth_sum_inverse(nested_list: Vec<NestedInteger>) -> i32 {
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

impl NestedInteger {
    pub fn parse_int(input: &mut &str) -> Option<NestedInteger> {
        let mut j = 0;
        for (i, c) in input.char_indices() {
            if !c.is_numeric() {
                j = i;
                break;
            }
        }
        if let Ok(x) = input[0..j].parse::<i32>() {
            *input = &input[j..];
            return Some(NestedInteger::Int(x));
        }
        None
    }

    pub fn parse_list(input: &mut &str) -> Option<NestedInteger> {
        if !Self::eat(input, '[') {
            return Self::parse_int(input);
        }
        let mut v: Vec<NestedInteger> = vec![];
        while let Some(x) = Self::parse_list(input) {
            v.push(x);
            Self::eat(input, ',');
        }
        if !Self::eat(input, ']') {
            return None;
        }
        Some(NestedInteger::List(v))
    }

    pub fn eat(input: &mut &str, c: char) -> bool {
        if let Some(x) = input.chars().next() {
            if x == c {
                *input = &input[1..];
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    use NestedInteger::*;
    let mut input = "[[1,1],2,[1,1]]";
    let nested_list = List(vec![
        List(vec![Int(1), Int(1)]),
        Int(2),
        List(vec![Int(1), Int(1)]),
    ]);
    assert_eq!(NestedInteger::parse_list(&mut input), Some(nested_list));

    let nested_list = List(vec![
        List(vec![Int(1), Int(1)]),
        Int(2),
        List(vec![Int(1), Int(1)]),
    ]);
    assert_eq!(Solution::depth_sum_inverse(vec![nested_list]), 8);
}
