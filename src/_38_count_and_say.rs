struct Solution {}

static mut s: Option<Vec<String>> = None;

fn count_and_say(x: &String) -> String {
    let n = x.len();
    let mut count = 0;
    let mut say = '0';
    let mut word = String::from("");
    for (i, c) in x.chars().enumerate() {
        if i == 0 {
            say = c;
            count = 1;
        } else {
            if say == c {
                count = count + 1;
            } else {
                word.push_str(&count.to_string());
                word.push_str(&say.to_string());
                say = c;
                count = 1;
            }
        }
        if i == n - 1 {
            word.push_str(&count.to_string());
            word.push_str(&say.to_string());
        }
    }
    word
}

unsafe fn init() -> Option<Vec<String>> {
    let mut v: Vec<String> = vec![String::from("1")];
    for i in 1..31 {
        let word = count_and_say(&v[i - 1]);
        v.push(word);
    }
    Some(v)
}

impl Solution {
    fn count_and_say(n: i32) -> String {
        unsafe {
            let i: usize = n as usize - 1;
            if None == s {
                s = init();
            }
            match &s {
                Some(v) => v[i].clone(),
                None => String::from("xxx"),
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_and_say(4), String::from("1211"));
    assert_eq!(Solution::count_and_say(1), String::from("1"));
}
