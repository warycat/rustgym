struct Solution;

struct Folder {
    level: usize,
    length: usize,
}

impl Solution {
    fn length_longest_path(input: String) -> i32 {
        let mut stack: Vec<Folder> = vec![];
        let mut folder_length = 0;
        let mut max = 0;
        for line in input.split('\n') {
            let s = line.chars();
            let mut level: usize = 0;
            let mut is_folder = true;
            for c in s {
                match c {
                    '\t' => {
                        level += 1;
                    }
                    '.' => {
                        is_folder = false;
                        break;
                    }
                    _ => {}
                }
            }
            let name = line[level..].to_string();
            while let Some(top) = stack.pop() {
                if top.level >= level {
                    folder_length -= top.length;
                } else {
                    stack.push(top);
                    break;
                }
            }
            if is_folder {
                let length = name.len() + 1;
                let folder = Folder { level, length };
                stack.push(folder);
                folder_length += length;
            } else {
                max = max.max(name.len() + folder_length);
            }
        }
        max as i32
    }
}

#[test]
fn test() {
    let input =
        "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
            .to_string();
    let res = 32;
    assert_eq!(Solution::length_longest_path(input), res);
}
