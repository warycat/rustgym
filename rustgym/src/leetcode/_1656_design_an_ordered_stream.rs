use std::collections::HashMap;
struct OrderedStream {
    ptr: i32,
    pairs: HashMap<i32, String>,
    n: i32,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        let pairs = HashMap::new();
        let ptr = 1;
        OrderedStream { ptr, pairs, n }
    }

    fn insert(&mut self, id: i32, value: String) -> Vec<String> {
        self.pairs.insert(id, value);
        let mut res = vec![];
        if self.ptr == id {
            for i in id..=self.n {
                if let Some(v) = self.pairs.get(&i) {
                    res.push(v.to_string());
                    self.ptr += 1;
                } else {
                    break;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let mut obj = OrderedStream::new(5);
    assert_eq!(obj.insert(3, "ccccc".to_string()), vec_string![]);
    assert_eq!(obj.insert(1, "aaaaa".to_string()), vec_string!["aaaaa"]);
    assert_eq!(
        obj.insert(2, "bbbbb".to_string()),
        vec_string!["bbbbb", "ccccc"]
    );
    assert_eq!(obj.insert(5, "eeeee".to_string()), vec_string![]);
    assert_eq!(
        obj.insert(4, "ddddd".to_string()),
        vec_string!["ddddd", "eeeee"]
    );
}
