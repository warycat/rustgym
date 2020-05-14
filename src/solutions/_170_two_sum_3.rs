use std::collections::HashMap;
use std::i32;

#[derive(Default)]
pub struct TwoSum {
    pub numbers: HashMap<i32, usize>,
    pub max: i32,
    pub min: i32,
}

impl TwoSum {
    pub fn new() -> Self {
        TwoSum {
            numbers: HashMap::new(),
            max: i32::MIN,
            min: i32::MAX,
        }
    }

    pub fn add(&mut self, number: i32) {
        self.max = i32::max(self.max, number << 1);
        self.min = i32::min(self.min, number << 1);
        self.numbers
            .insert(number, self.numbers.get(&number).unwrap_or(&0) + 1);
    }

    pub fn find(&self, value: i32) -> bool {
        if value < self.min || value > self.max {
            return false;
        }
        for (&a, &v) in &self.numbers {
            let b = value - a;
            if a == b && v == 2 {
                return true;
            }
            if a != b && self.numbers.contains_key(&b) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let mut two_some = TwoSum::new();
    two_some.add(1);
    two_some.add(3);
    two_some.add(5);
    assert_eq!(two_some.find(4), true);
    assert_eq!(two_some.find(7), false);
}
