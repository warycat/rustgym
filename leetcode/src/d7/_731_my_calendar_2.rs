use std::collections::BTreeMap;

#[derive(Default)]
struct MyCalendarTwo {
    double_booked: Vec<Interval>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        let double_booked = vec![];
        MyCalendarTwo { double_booked }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let mut triple_booked = MyCalendar::new();
        for &(a, b) in &self.double_booked {
            let a = a.max(start);
            let b = b.min(end);
            if a < b && !triple_booked.book(a, b) {
                return false;
            }
        }
        self.double_booked.push((start, end));
        true
    }
}
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
    let mut obj = MyCalendarTwo::new();
    assert_eq!(obj.book(10, 20), true);
    assert_eq!(obj.book(50, 60), true);
    assert_eq!(obj.book(10, 40), true);
    assert_eq!(obj.book(5, 15), false);
    assert_eq!(obj.book(5, 10), true);
    assert_eq!(obj.book(25, 55), true);
}
