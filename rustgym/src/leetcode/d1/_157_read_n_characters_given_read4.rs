use std::cell::RefCell;
use std::rc::Rc;

struct Solution {
    file: Vec<char>,
    index: Rc<RefCell<usize>>,
}

impl Solution {
    fn new(file: String) -> Self {
        let file = file.chars().collect();
        let index = Rc::new(RefCell::new(0));
        Solution { file, index }
    }

    fn read4(&self, buf4: &mut [char]) -> i32 {
        for i in 0..4 {
            if *self.index.borrow() == self.file.len() {
                return i as i32;
            }
            buf4[i] = self.file[*self.index.borrow()];
            *self.index.borrow_mut() += 1;
        }
        4
    }
}

impl Solution {
    fn read(&self, buf: &mut [char], n: i32) -> i32 {
        let n = n as usize;
        let mut buf4: Vec<char> = vec![' '; 4];
        let mut i = 0;
        let mut j = 0;
        let mut m = 0;
        while i < n {
            if m == 0 {
                j = 0;
                m = self.read4(&mut buf4);
                if m == 0 {
                    break;
                }
            } else {
                buf[i] = buf4[j];
                i += 1;
                j += 1;
                m -= 1;
            }
        }
        i as i32
    }
}

#[test]
fn test() {
    let obj = Solution::new("abc".to_string());
    let n = 4;
    let mut buf = vec![' '; 100];
    let res = 3;
    assert_eq!(obj.read(&mut buf, n), res);
    let obj = Solution::new("abcde".to_string());
    let n = 5;
    let mut buf = vec![' '; 100];
    let res = 5;
    assert_eq!(obj.read(&mut buf, n), res);
    let obj = Solution::new("abcdABCD1234".to_string());
    let n = 12;
    let mut buf = vec![' '; 100];
    let res = 12;
    assert_eq!(obj.read(&mut buf, n), res);
    let obj = Solution::new("leetcode".to_string());
    let n = 5;
    let mut buf = vec![' '; 100];
    let res = 5;
    assert_eq!(obj.read(&mut buf, n), res);
}
