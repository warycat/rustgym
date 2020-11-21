mod hello_world;
mod sock_merchant;

use std::fmt::Write;
use std::io::*;

fn solve(_: &mut dyn BufRead, _: &mut dyn Write) {}

fn main() {
    let mut res = "".to_string();
    solve(&mut BufReader::new(stdin()), &mut res);
    print!("{}", res);
}
