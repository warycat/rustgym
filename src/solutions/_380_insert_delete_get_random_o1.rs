extern crate rand;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct RandomizedSet {
    pub rng: ThreadRng,
    pub indexes: HashMap<i32, usize>,
    pub values: Vec<i32>,
}

impl RandomizedSet {
    pub fn new() -> Self {
        RandomizedSet {
            rng: rand::thread_rng(),
            indexes: HashMap::new(),
            values: vec![],
        }
    }
    pub fn insert(&mut self, val: i32) -> bool {
        if self.indexes.get(&val).is_some() {
            false
        } else {
            self.indexes.insert(val, self.values.len());
            self.values.push(val);
            true
        }
    }
    pub fn remove(&mut self, val: i32) -> bool {
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
    pub fn get_random(&mut self) -> i32 {
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
