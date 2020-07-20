use std::collections::HashMap;

type Pair = (i32, String);

#[derive(Default)]
struct TimeMap {
    map: HashMap<String, Vec<Pair>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }
    fn get(&mut self, key: String, timestamp: i32) -> String {
        let values = self.map.entry(key).or_default();
        match values.binary_search_by_key(&timestamp, |v| v.0) {
            Ok(i) => values[i].1.to_string(),
            Err(i) => {
                if i > 0 {
                    values[i - 1].1.to_string()
                } else {
                    "".to_string()
                }
            }
        }
    }
}

#[test]
fn test() {
    let mut kv: TimeMap = TimeMap::new();
    kv.set("foo".to_string(), "bar".to_string(), 1);
    assert_eq!(kv.get("foo".to_string(), 1), "bar".to_string());
    assert_eq!(kv.get("foo".to_string(), 3), "bar".to_string());
    kv.set("foo".to_string(), "bar2".to_string(), 4);
    assert_eq!(kv.get("foo".to_string(), 4), "bar2".to_string());
    assert_eq!(kv.get("foo".to_string(), 5), "bar2".to_string());
}
