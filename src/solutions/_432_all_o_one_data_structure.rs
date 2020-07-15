use std::collections::HashMap;

struct AllOne {
    dict: HashMap<String, usize>,
}

impl AllOne {
    fn new() -> Self {
        let dict = HashMap::new();
        AllOne { dict }
    }

    fn inc(&mut self, key: String) {
        self.dict.entry(key).and_modify(|v| *v += 1).or_insert(1);
    }

    fn dec(&mut self, key: String) {
        if !self.dict.contains_key(&key) {
            return;
        }
        if self.dict[&key] == 1 {
            self.dict.remove(&key);
        } else {
            *self.dict.get_mut(&key).unwrap() -= 1;
        }
    }

    fn get_max_key(&mut self) -> String {
        self.dict
            .iter()
            .max_by_key(|(_, &v)| v)
            .unwrap_or((&String::from(""), &0))
            .0
            .to_string()
    }

    fn get_min_key(&mut self) -> String {
        self.dict
            .iter()
            .min_by_key(|(_, &v)| v)
            .unwrap_or((&String::from(""), &0))
            .0
            .to_string()
    }
}

#[test]
fn test() {
    let mut obj = AllOne::new();
    obj.inc("abc".to_string());
    assert_eq!(obj.get_max_key(), "abc".to_string());
}
