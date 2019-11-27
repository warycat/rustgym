use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

struct RandomizedSet {
    indexes: HashMap<i32, usize>,
    values: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            indexes: HashMap::new(),
            values: vec![],
        }
    }
    fn insert(&mut self, val: i32) -> bool {
        if self.indexes.get(&val).is_some() {
            false
        } else {
            self.indexes.insert(val, self.values.len());
            self.values.push(val);
            true
        }
    }
    fn remove(&mut self, val: i32) -> bool {
        if let Some(index) = self.indexes.remove(&val) {
            let last = self.values[self.values.len() - 1];
            self.indexes
                .entry(last)
                .and_modify(|old_index| *old_index = index);
            self.values[index] = last;
            self.values.pop();
            true
        } else {
            false
        }
    }
    fn get_random(&self) -> i32 {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();

        self.values[nanos as usize % self.values.len()]
    }
}

#[test]
fn test() {
    let mut obj = RandomizedSet::new();
    assert_eq!(obj.insert(1), true);
    assert_eq!(obj.remove(2), false);
    assert_eq!(obj.insert(2), true);
    assert_eq!(obj.remove(1), true);
    assert_eq!(obj.insert(2), false);
}
