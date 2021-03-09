extern crate rand;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
struct RandomizedSet {
    rng: ThreadRng,
    indexes: HashMap<i32, usize>,
    values: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            rng: rand::thread_rng(),
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
            let last_index = self.values.len() - 1;
            let last_value = self.values[last_index];
            if index != last_index {
                self.values.swap(index, last_index);
                let old_index = self.indexes.get_mut(&last_value).unwrap();
                *old_index = index;
            }
            self.values.pop();
            true
        } else {
            false
        }
    }

    fn get_random(&mut self) -> i32 {
        let index = self.rng.gen_range(0, self.values.len()) as usize;
        self.values[index]
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
