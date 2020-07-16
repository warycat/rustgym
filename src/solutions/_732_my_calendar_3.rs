use std::collections::BTreeMap;

struct MyCalendarThree {
    data: BTreeMap<i32, i32>,
}

impl MyCalendarThree {
    fn new() -> Self {
        let data = BTreeMap::new();
        MyCalendarThree { data }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.data.entry(start).or_default() += 1;
        *self.data.entry(end).or_default() -= 1;
        let mut cur = 0;
        let mut res = 0;
        for v in self.data.values() {
            cur += v;
            res = res.max(cur);
        }
        res
    }
}

#[test]
fn test() {
    let mut obj = MyCalendarThree::new();
    assert_eq!(obj.book(10, 20), 1);
    assert_eq!(obj.book(50, 60), 1);
    assert_eq!(obj.book(10, 40), 2);
    assert_eq!(obj.book(5, 15), 3);
    assert_eq!(obj.book(5, 10), 3);
    assert_eq!(obj.book(25, 55), 3);
}
