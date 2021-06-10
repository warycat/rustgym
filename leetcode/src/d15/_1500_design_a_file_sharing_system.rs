use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::iter::FromIterator;

struct FileSharing {
    ids: BinaryHeap<Reverse<i32>>,
    files: BTreeMap<i32, HashSet<i32>>,
    m: i32,
}

impl FileSharing {
    fn new(m: i32) -> Self {
        let ids = BinaryHeap::new();
        let files = BTreeMap::new();
        FileSharing { ids, files, m }
    }

    fn join(&mut self, owned_chunks: Vec<i32>) -> i32 {
        let id: i32 = if let Some(Reverse(min)) = self.ids.pop() {
            min
        } else {
            self.files.len() as i32 + 1
        };
        let hs = HashSet::from_iter(owned_chunks);
        self.files.insert(id, hs);
        id
    }

    fn leave(&mut self, user_id: i32) {
        self.files.remove(&user_id);
        self.ids.push(Reverse(user_id));
    }

    fn request(&mut self, user_id: i32, chunk_id: i32) -> Vec<i32> {
        let mut res = vec![];
        for (&id, set) in &self.files {
            if set.contains(&chunk_id) {
                res.push(id);
            }
        }
        if !res.is_empty() {
            self.files.get_mut(&user_id).unwrap().insert(chunk_id);
        }
        res
    }
}

#[test]
fn test() {
    let mut obj = FileSharing::new(4);
    assert_eq!(obj.join(vec![1, 2]), 1);
    assert_eq!(obj.join(vec![2, 3]), 2);
    assert_eq!(obj.join(vec![4]), 3);
    assert_eq!(obj.request(1, 3), vec![2]);
    assert_eq!(obj.request(2, 2), vec![1, 2]);
    obj.leave(1);
    let res: Vec<i32> = vec![];
    assert_eq!(obj.request(2, 1), res);
    obj.leave(2);
    assert_eq!(obj.join(vec![]), 1);
}
