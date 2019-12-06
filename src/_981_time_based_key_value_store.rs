use std::collections::HashMap;

struct TimeValue {
    value: String,
    timestamp: i32,
}

struct TimeMap {
    map: HashMap<String, Vec<TimeValue>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map
            .entry(key)
            .or_default()
            .push(TimeValue { value, timestamp });
    }
    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(values) = self.map.get(&key) {
            match values.binary_search_by_key(&timestamp, |v| v.timestamp) {
                Ok(i) => values[i].value.to_string(),
                Err(i) => {
                    if i > 0 {
                        values[i - 1].value.to_string()
                    } else {
                        "".to_string()
                    }
                }
            }
        } else {
            "".to_string()
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
