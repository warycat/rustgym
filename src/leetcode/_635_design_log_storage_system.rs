use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Default)]
struct LogSystem {
    min: String,
    max: String,
    map: HashMap<String, usize>,
    logs: BTreeMap<String, i32>,
}

impl LogSystem {
    fn new() -> Self {
        let min = "2000:01:01:00:00:00".to_string();
        let max = "2017:12:31:23:59:59".to_string();
        let map: HashMap<String, usize> = [
            ("Year", 4),
            ("Month", 7),
            ("Day", 10),
            ("Hour", 13),
            ("Minute", 16),
            ("Second", 19),
        ]
        .iter()
        .map(|(s, i)| ((*s).to_string(), *i))
        .collect();
        let logs: BTreeMap<String, i32> = BTreeMap::new();
        LogSystem {
            min,
            max,
            map,
            logs,
        }
    }

    fn put(&mut self, id: i32, timestamp: String) {
        *self.logs.entry(timestamp).or_default() = id;
    }

    fn retrieve(&self, s: String, e: String, gra: String) -> Vec<i32> {
        let i = self.map[&gra];
        let s = "".to_string() + &s[0..i] + &self.min[i..];
        let e = "".to_string() + &e[0..i] + &self.max[i..];
        self.logs.range(s..=e).map(|(_, &i)| i).collect()
    }
}

#[test]
fn test() {
    let mut obj = LogSystem::new();
    obj.put(1, "2017:01:01:23:59:59".to_string());
    obj.put(2, "2017:01:01:22:59:59".to_string());
    obj.put(3, "2016:01:01:00:00:00".to_string());
    let s = "2016:01:01:01:01:01".to_string();
    let e = "2017:01:01:23:00:00".to_string();
    let gra = "Year".to_string();
    let ans = vec![1, 2, 3];
    let mut res = obj.retrieve(s, e, gra);
    res.sort_unstable();
    assert_eq!(ans, res);
    let s = "2016:01:01:01:01:01".to_string();
    let e = "2017:01:01:23:00:00".to_string();
    let gra = "Hour".to_string();
    let ans = vec![1, 2];
    let mut res = obj.retrieve(s, e, gra);
    res.sort_unstable();
    assert_eq!(ans, res);
}
