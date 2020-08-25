use std::collections::BTreeMap;
use std::collections::HashSet;

struct SummaryRanges {
    data: BTreeMap<i32, i32>,
    seen: HashSet<i32>,
}

impl SummaryRanges {
    fn new() -> Self {
        let data = BTreeMap::new();
        let seen = HashSet::new();
        SummaryRanges { data, seen }
    }

    fn add_num(&mut self, val: i32) {
        if !self.seen.insert(val) {
            return;
        }
        let mut l = val;
        let mut r = val;
        if let Some(&right) = self.data.get(&(val + 1)) {
            r = right;
        }
        if let Some((&left, &limit)) = self.data.range(..val).rev().next() {
            if limit == val - 1 {
                l = left;
            }
        }
        if l < val {
            self.data.remove(&l);
        }
        if r > val {
            self.data.remove(&(val + 1));
        }
        self.data.insert(l, r);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.data.iter().map(|(&k, &v)| vec![k, v]).collect()
    }
}

#[test]
fn test() {
    let mut obj = SummaryRanges::new();
    obj.add_num(1);
    obj.add_num(3);
    obj.add_num(7);
    obj.add_num(2);
    obj.add_num(6);
    assert_eq!(obj.get_intervals(), vec![vec![1, 3], vec![6, 7]]);
}
