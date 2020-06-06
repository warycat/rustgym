use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hasher;

struct MapSum {
    sum: HashMap<u64, i32>,
    val: HashMap<u64, i32>,
}

impl MapSum {
    fn new() -> Self {
        let sum = HashMap::new();
        let val = HashMap::new();
        MapSum { sum, val }
    }

    fn insert(&mut self, key: String, val: i32) {
        let mut hasher = DefaultHasher::new();
        for b in key.bytes() {
            hasher.write_u8(b);
            let k = hasher.finish();
            *self.sum.entry(k).or_default() += val;
        }
        let k = hasher.finish();
        if let Some(prev) = self.val.insert(k, val) {
            let mut hasher = DefaultHasher::new();
            for b in key.bytes() {
                hasher.write_u8(b);
                let k = hasher.finish();
                *self.sum.entry(k).or_default() -= prev;
            }
        }
    }

    fn sum(&mut self, prefix: String) -> i32 {
        let mut hasher = DefaultHasher::new();
        for b in prefix.bytes() {
            hasher.write_u8(b);
        }
        let k = hasher.finish();
        *self.sum.entry(k).or_default()
    }
}

#[test]
fn test() {
    let mut obj = MapSum::new();
    obj.insert("apple".to_string(), 3);
    assert_eq!(obj.sum("ap".to_string()), 3);
    obj.insert("app".to_string(), 2);
    assert_eq!(obj.sum("ap".to_string()), 5);
}
