mod hello_world;
mod jumping_on_the_clouds;
mod repeated_string;
mod sock_merchant;

use std::fmt::Write;
use std::io::*;

fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let mut it = reader.lines().map(|l| l.unwrap());
    let line = it.next().unwrap();
    write!(writer, "{}", line).unwrap();
}

fn main() {
    let mut res = "".to_string();
    solve(&mut BufReader::new(stdin()), &mut res);
    print!("{}", res);
}
