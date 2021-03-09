use rustgym_util::*;
use std::iter::Peekable;
use std::vec::IntoIter;

struct Codec;

enum Tok {
    Op(char),
    Num(i32),
}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: TreeLink) -> String {
        let mut res = "".to_string();
        Self::serialize_tree(&root, &mut res);
        res
    }

    fn serialize_tree(root: &TreeLink, s: &mut String) {
        s.push('(');
        if let Some(node) = root {
            let node = node.borrow();
            *s += &format!("{}", node.val);
            Self::serialize_tree(&node.left, s);
            Self::serialize_tree(&node.right, s);
        }
        s.push(')');
    }

    fn deserialize(&self, data: String) -> TreeLink {
        let tokens = Self::parse_tokens(data);
        let mut it = tokens.into_iter().peekable();
        Self::parse_tree(&mut it)
    }

    fn parse_tokens(data: String) -> Vec<Tok> {
        let mut it = data.chars().peekable();
        let mut res = vec![];
        while let Some(c) = it.next() {
            if c == '(' || c == ')' {
                res.push(Tok::Op(c));
            } else {
                let mut x = (c as u8 - b'0') as i32;
                while let Some('0'..='9') = it.peek() {
                    x *= 10;
                    x += (it.next().unwrap() as u8 - b'0') as i32;
                }
                res.push(Tok::Num(x));
            }
        }
        res
    }

    fn parse_tree(it: &mut Peekable<IntoIter<Tok>>) -> TreeLink {
        let mut res = None;
        it.next();
        match it.peek() {
            Some(&Tok::Num(x)) => {
                it.next();
                res = tree!(x, Self::parse_tree(it), Self::parse_tree(it))
            }
            Some(Tok::Op(')')) => {}
            _ => panic!(),
        }
        it.next();
        res
    }
}

#[test]
fn test() {
    let codec = Codec::new();
    let root = tree!(2, tree!(1), tree!(3));
    let ans = tree!(2, tree!(1), tree!(3));
    assert_eq!(codec.deserialize(codec.serialize(root)), ans);
}
