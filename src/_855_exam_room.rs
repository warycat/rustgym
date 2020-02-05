use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::HashMap;

// (distance, left, right)
type Segment = (Reverse<i32>, i32, i32);

#[derive(Debug)]
struct ExamRoom {
    n: i32,
    segments: BTreeSet<Segment>,
    l_indexes: HashMap<i32, i32>,
    r_indexes: HashMap<i32, i32>,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        let mut segments = BTreeSet::new();
        segments.insert(Self::segment(0, n - 1, n));
        let mut l_indexes = HashMap::new();
        let mut r_indexes = HashMap::new();
        l_indexes.insert(0, n - 1);
        r_indexes.insert(n - 1, 0);
        ExamRoom {
            n,
            segments,
            l_indexes,
            r_indexes,
        }
    }

    fn seat(&mut self) -> i32 {
        let mut it = self.segments.iter();
        if let Some(&first) = it.next() {
            let l = first.1;
            let r = first.2;
            let p = Self::split(&first, self.n);
            self.segments.remove(&first);
            self.segments.insert(Self::segment(l, p - 1, self.n));
            self.segments.insert(Self::segment(p + 1, r, self.n));
            self.l_indexes.insert(l, p - 1);
            self.r_indexes.insert(p - 1, l);
            self.l_indexes.insert(p + 1, r);
            self.r_indexes.insert(r, p + 1);
            p
        } else {
            -1
        }
    }

    fn leave(&mut self, p: i32) {
        let r1 = p - 1;
        let l1 = self.r_indexes[&r1];
        let l2 = p + 1;
        let r2 = self.l_indexes[&l2];
        self.segments.remove(&Self::segment(l1, r1, self.n));
        self.segments.remove(&Self::segment(l2, r2, self.n));
        self.segments.insert(Self::segment(l1, r2, self.n));
        self.r_indexes.remove(&r1);
        self.l_indexes.remove(&l2);
        self.l_indexes.insert(l1, r2);
        self.r_indexes.insert(r2, l1);
    }

    fn segment(l: i32, r: i32, n: i32) -> Segment {
        if l == 0 {
            return (Reverse(r), l, r);
        }
        if r == n - 1 {
            return (Reverse(n - 1 - l), l, r);
        }
        if l <= r {
            (Reverse((r - l) / 2), l, r)
        } else {
            (Reverse(-1), l, r)
        }
    }

    fn split(s: &Segment, n: i32) -> i32 {
        let l = s.1;
        let r = s.2;
        if l == 0 {
            return 0;
        }
        if r == n - 1 {
            return n - 1;
        }
        l + (r - l) / 2
    }
}

#[test]
fn test() {
    let mut exam_room = ExamRoom::new(10);
    assert_eq!(exam_room.seat(), 0);
    assert_eq!(exam_room.seat(), 9);
    assert_eq!(exam_room.seat(), 4);
    assert_eq!(exam_room.seat(), 2);
    exam_room.leave(4);
    assert_eq!(exam_room.seat(), 5);

    let mut exam_room = ExamRoom::new(4);
    assert_eq!(exam_room.seat(), 0);
    assert_eq!(exam_room.seat(), 3);
    assert_eq!(exam_room.seat(), 1);
    assert_eq!(exam_room.seat(), 2);
    exam_room.leave(1);
    exam_room.leave(3);
    assert_eq!(exam_room.seat(), 1);
}
