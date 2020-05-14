type Pair = (usize, i32);

struct SnapshotArray {
    size: usize,
    data: Vec<Vec<Pair>>,
    snap_id: usize,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        let size = length as usize;
        let data = vec![vec![(0, 0)]; size];
        let snap_id = 0;
        SnapshotArray {
            size,
            data,
            snap_id,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        let i = index as usize;
        if let Some(last) = self.data[i].pop() {
            if last.0 != self.snap_id {
                self.data[i].push(last);
            }
        }
        self.data[i].push((self.snap_id, val));
    }

    fn snap(&mut self) -> i32 {
        self.snap_id += 1;
        (self.snap_id - 1) as i32
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let i = index as usize;
        let snap_id = snap_id as usize;
        match self.data[i].binary_search_by_key(&snap_id, |p| p.0) {
            Ok(j) => self.data[i][j].1,
            Err(j) => self.data[i][j - 1].1,
        }
    }
}

#[test]
fn test() {
    let mut obj = SnapshotArray::new(3);
    obj.set(0, 5);
    assert_eq!(obj.snap(), 0);
    obj.set(0, 6);
    assert_eq!(obj.get(0, 0), 5);
}
