#[derive(Debug, Clone)]
enum Cell {
    Value(i32),
    Sum(Vec<(usize, usize)>),
}
struct Excel {
    mat: Vec<Vec<Cell>>,
}

impl Excel {
    fn new(h: i32, w: char) -> Self {
        let n = h as usize;
        let m = (w as u8 - b'A' + 1) as usize;
        let mat = vec![vec![Cell::Value(0); m]; n];
        Excel { mat }
    }

    fn set(&mut self, r: i32, c: char, v: i32) {
        let i = Self::row(r);
        let j = Self::col(c);
        self.mat[i][j] = Cell::Value(v);
    }

    fn get(&self, r: i32, c: char) -> i32 {
        let i = Self::row(r);
        let j = Self::col(c);
        self.get_by_index(i, j)
    }

    fn get_by_index(&self, i: usize, j: usize) -> i32 {
        match &self.mat[i][j] {
            Cell::Value(v) => *v,
            Cell::Sum(nums) => nums.iter().map(|&(i, j)| self.get_by_index(i, j)).sum(),
        }
    }

    fn sum(&mut self, r: i32, c: char, strs: Vec<String>) -> i32 {
        let i = Self::row(r);
        let j = Self::col(c);
        let mut nums = vec![];
        for s in strs {
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() == 1 {
                nums.push(Self::cell(parts[0]));
            } else {
                let top_left = Self::cell(parts[0]);
                let bottom_right = Self::cell(parts[1]);
                for i in top_left.0..=bottom_right.0 {
                    for j in top_left.1..=bottom_right.1 {
                        nums.push((i, j));
                    }
                }
            }
        }
        self.mat[i][j] = Cell::Sum(nums);
        self.get_by_index(i, j)
    }

    fn row(r: i32) -> usize {
        (r - 1) as usize
    }

    fn col(c: char) -> usize {
        (c as u8 - b'A') as usize
    }

    fn cell(s: &str) -> (usize, usize) {
        let mut col = 0;
        let mut row = 0;
        for (i, c) in s.char_indices() {
            if i == 0 {
                col = (c as u8 - b'A') as usize;
            } else {
                row *= 10;
                row += (c as u8 - b'0') as usize;
            }
        }
        row -= 1;
        (row, col)
    }
}

#[test]
fn test() {
    let mut obj = Excel::new(3, 'C');
    obj.set(1, 'A', 2);
    assert_eq!(obj.get(1, 'A'), 2);
    assert_eq!(obj.sum(3, 'C', vec_string!["A1", "A1:B2"]), 4);
    obj.set(2, 'B', 2);
    assert_eq!(obj.get(3, 'C'), 6);
}
