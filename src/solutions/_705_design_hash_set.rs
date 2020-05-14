#[derive(Default)]
pub struct MyHashSet {
    pub table: Vec<bool>,
}

impl MyHashSet {
    pub fn new() -> Self {
        MyHashSet {
            table: vec![false; 1_000_000],
        }
    }
    pub fn add(&mut self, key: i32) {
        self.table[key as usize] = true;
    }
    pub fn remove(&mut self, key: i32) {
        self.table[key as usize] = false;
    }
    pub fn contains(&self, key: i32) -> bool {
        self.table[key as usize]
    }
}

#[test]
fn test() {
    let mut hs = MyHashSet::new();
    hs.add(1);
    hs.add(2);
    assert_eq!(hs.contains(1), true);
    assert_eq!(hs.contains(3), false);
    hs.add(2);
    assert_eq!(hs.contains(2), true);
    hs.remove(2);
    assert_eq!(hs.contains(2), false);
}
