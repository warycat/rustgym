struct Solution;
use rustgym_util::*;
use std::iter::Peekable;
use std::str::Chars;
use std::vec::IntoIter;

enum Tok {
    N(i32),
    D(usize),
}

trait Preorder {
    fn parse(it: &mut Peekable<IntoIter<Tok>>, depth: usize) -> TreeLink;
    fn parse_root(it: &mut Peekable<IntoIter<Tok>>) -> TreeLink;
}

impl Preorder for TreeLink {
    fn parse(it: &mut Peekable<IntoIter<Tok>>, depth: usize) -> TreeLink {
        if let Some(&Tok::D(d)) = it.peek() {
            if d == depth {
                it.next();
                if let Some(Tok::N(n)) = it.next() {
                    TreeLink::branch(n, TreeLink::parse(it, d + 1), TreeLink::parse(it, d + 1))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_root(it: &mut Peekable<IntoIter<Tok>>) -> TreeLink {
        if let Some(Tok::N(n)) = it.next() {
            TreeLink::branch(n, TreeLink::parse(it, 1), TreeLink::parse(it, 1))
        } else {
            None
        }
    }
}

impl Solution {
    fn recover_from_preorder(s: String) -> TreeLink {
        let toks: Vec<Tok> = Self::parse_tokens(&mut s.chars().peekable());
        TreeLink::parse_root(&mut toks.into_iter().peekable())
    }

    fn parse_tokens(it: &mut Peekable<Chars>) -> Vec<Tok> {
        let mut toks: Vec<Tok> = vec![];
        while let Some(c) = it.next() {
            match c {
                '-' => {
                    let mut d = 1;
                    while let Some('-') = it.peek() {
                        it.next();
                        d += 1;
                    }
                    toks.push(Tok::D(d));
                }
                '0'..='9' => {
                    let mut n = (c as u8 - b'0') as i32;
                    while let Some('0'..='9') = it.peek() {
                        n *= 10;
                        n += (it.next().unwrap() as u8 - b'0') as i32;
                    }
                    toks.push(Tok::N(n));
                }
                _ => {}
            }
        }
        toks
    }
}

#[test]
fn test() {
    let s = "1-2--3--4-5--6--7".to_string();
    let res = tree!(
        1,
        tree!(2, tree!(3), tree!(4)),
        tree!(5, tree!(6), tree!(7))
    );
    assert_eq!(Solution::recover_from_preorder(s), res);
    let s = "1-2--3---4-5--6---7".to_string();
    let res = tree!(
        1,
        tree!(2, tree!(3, tree!(4), None), None),
        tree!(5, tree!(6, tree!(7), None), None)
    );
    assert_eq!(Solution::recover_from_preorder(s), res);
    let s = "1-401--349---90--88".to_string();
    let res = tree!(1, tree!(401, tree!(349, tree!(90), None), tree!(88)), None);
    assert_eq!(Solution::recover_from_preorder(s), res);
}
