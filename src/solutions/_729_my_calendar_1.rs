use std::collections::BTreeMap;

type Interval = (i32, i32);
type Center = i32;

#[derive(Default)]
struct MyCalendar {
    btm: BTreeMap<Center, Interval>,
}

impl MyCalendar {
    fn new() -> Self {
        let btm: BTreeMap<Center, Interval> = BTreeMap::new();
        MyCalendar { btm }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let center = start + end;
        if let Some((_, &(_, last_end))) = self.btm.range(..=center).next_back() {
            if start < last_end {
                return false;
            }
        }
        if let Some((_, &(first_start, _))) = self.btm.range(center..).next() {
            if first_start < end {
                return false;
            }
        }
        self.btm.insert(center, (start, end));
        true
    }
}

#[test]
fn test() {
    let mut obj = MyCalendar::new();
    assert_eq!(obj.book(10, 20), true);
    assert_eq!(obj.book(15, 25), false);
    assert_eq!(obj.book(20, 30), true);
}
