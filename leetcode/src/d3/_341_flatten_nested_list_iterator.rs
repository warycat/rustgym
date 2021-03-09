use rustgym_util::*;
use std::iter::Peekable;
use std::vec::IntoIter;

struct NestedIterator {
    it: Peekable<IntoIter<i32>>,
}

trait ToVec {
    fn to_vec(&self) -> Vec<i32>;
}

impl ToVec for NestedInteger {
    fn to_vec(&self) -> Vec<i32> {
        match self {
            NestedInteger::Int(x) => vec![*x],
            NestedInteger::List(v) => {
                let mut res = vec![];
                for x in v {
                    res.append(&mut x.to_vec());
                }
                res
            }
        }
    }
}

impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut data = vec![];
        for x in nested_list {
            data.append(&mut x.to_vec());
        }

        NestedIterator {
            it: data.into_iter().peekable(),
        }
    }

    fn next(&mut self) -> i32 {
        self.it.next().unwrap()
    }

    fn has_next(&mut self) -> bool {
        self.it.peek().is_some()
    }
}

#[test]
fn test() {
    let nested_list = vec![
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        NestedInteger::Int(2),
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
    ];
    let mut obj = NestedIterator::new(nested_list);
    let res = vec![1, 1, 2, 1, 1];
    let mut ans = vec![];
    while obj.has_next() {
        ans.push(obj.next());
    }
    assert_eq!(ans, res);

    let nested_list = vec![
        NestedInteger::Int(1),
        NestedInteger::List(vec![
            NestedInteger::Int(4),
            NestedInteger::List(vec![NestedInteger::Int(6)]),
        ]),
    ];
    let mut obj = NestedIterator::new(nested_list);
    let res = vec![1, 4, 6];
    let mut ans = vec![];
    while obj.has_next() {
        ans.push(obj.next());
    }
    assert_eq!(ans, res);
}
