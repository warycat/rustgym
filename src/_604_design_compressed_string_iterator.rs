struct StringIterator {
    index: usize,
    pairs: Vec<Pair>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Pair {
    c: char,
    m: usize,
}

impl Pair {
    fn new(c: char, m: usize) -> Self {
        Pair { c, m }
    }
}

impl StringIterator {
    fn new(compressed_string: String) -> Self {
        let s: Vec<char> = compressed_string.chars().collect();
        let n = s.len();
        let mut i = 0;
        let mut pairs: Vec<Pair> = vec![];
        while i < n {
            let c = s[i];
            i += 1;
            let mut m: usize = 0;
            while i < n && s[i].is_numeric() {
                m *= 10;
                m += (s[i] as u8 - b'0') as usize;
                i += 1;
            }
            pairs.push(Pair::new(c, m));
        }
        StringIterator { index: 0, pairs }
    }

    fn next(&mut self) -> char {
        if self.index == self.pairs.len() {
            ' '
        } else {
            let c = self.pairs[self.index].c;
            self.pairs[self.index].m -= 1;
            if self.pairs[self.index].m == 0 {
                self.index += 1;
            }
            c
        }
    }

    fn has_next(&self) -> bool {
        self.index < self.pairs.len()
    }
}

#[test]
fn test() {
    let mut iterator = StringIterator::new("L1e2t1C1o1d1e1".to_string());
    assert_eq!(iterator.next(), 'L');
    assert_eq!(iterator.next(), 'e');
    assert_eq!(iterator.next(), 'e');
    assert_eq!(iterator.next(), 't');
    assert_eq!(iterator.next(), 'C');
    assert_eq!(iterator.next(), 'o');
    assert_eq!(iterator.next(), 'd');
    assert_eq!(iterator.has_next(), true);
    assert_eq!(iterator.next(), 'e');
    assert_eq!(iterator.has_next(), false);
    assert_eq!(iterator.next(), ' ');
}
