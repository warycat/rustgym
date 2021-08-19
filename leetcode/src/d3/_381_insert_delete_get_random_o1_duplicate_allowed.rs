use rand::prelude::*;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug)]
struct RandomizedCollection {
    rng: ThreadRng,
    indexes: HashMap<i32, BinaryHeap<usize>>,
    choices: Vec<i32>,
}

impl RandomizedCollection {
    fn new() -> Self {
        let rng = thread_rng();
        let indexes = HashMap::new();
        let choices = vec![];
        RandomizedCollection {
            rng,
            indexes,
            choices,
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        let ids = self.indexes.entry(val).or_default();
        ids.push(self.choices.len());
        self.choices.push(val);
        ids.len() == 1
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(ids) = self.indexes.get_mut(&val) {
            if let Some(index) = ids.pop() {
                let last_index = self.choices.len() - 1;
                let last_value = self.choices[last_index];
                if last_value != val {
                    let other_ids = self.indexes.get_mut(&last_value).unwrap();
                    other_ids.pop();
                    other_ids.push(index);
                    self.choices.swap(index, last_index);
                }
                self.choices.pop();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_random(&mut self) -> i32 {
        self.choices[self.rng.gen_range(0, self.choices.len())]
    }
}

#[test]
fn test() {
    let mut obj = RandomizedCollection::new();
    assert_eq!(obj.remove(0), false);
    // assert_eq!(obj.insert(1), false);
    // assert_eq!(obj.insert(2), true);
    // assert_eq!(obj.insert(2), false);
    // assert_eq!(obj.insert(2), false);
    // assert_eq!(obj.remove(1), true);
    // assert_eq!(obj.remove(1), true);
    // assert_eq!(obj.remove(2), true);
    // assert_eq!(obj.insert(1), true);
    // assert_eq!(obj.remove(2), true);
}
