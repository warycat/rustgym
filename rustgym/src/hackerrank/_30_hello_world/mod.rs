use rustgym_util::*;
use std::io::*;

fn solve(reader: &mut dyn BufRead) -> Result<String> {
    let mut res: String = "Hello, World.\n".to_string();
    reader.read_line(&mut res)?;
    Ok(res)
}

hackerrank!(test_0, "input00.txt", "output00.txt");
hackerrank!(test_1, "input01.txt", "output01.txt");
