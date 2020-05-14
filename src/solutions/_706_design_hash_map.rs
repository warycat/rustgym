#[derive(Default)]
pub struct MyHashMap {
    pub v: Vec<i32>,
}

const LIMIT: usize = 1_000_000;

impl MyHashMap {
    pub fn new() -> Self {
        MyHashMap {
            v: vec![-1; LIMIT + 1],
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.v[key as usize] = value;
    }

    pub fn get(&self, key: i32) -> i32 {
        self.v[key as usize]
    }

    pub fn remove(&mut self, key: i32) {
        self.v[key as usize] = -1;
    }
}
