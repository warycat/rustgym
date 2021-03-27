#![deny(clippy::all)]
#![allow(dead_code)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(unused_imports)]

mod hello_world;
mod jumping_on_the_clouds;
mod repeated_string;
mod sock_merchant;

use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(reader: &mut impl RustGymRead, writer: &mut impl Write) {
    let line: i32 = reader.parse_line();
    write!(writer, "{}", line).unwrap();
}

fn main() {
    let mut res = "".to_string();
    let mut reader = BufReader::new(stdin());
    solve(&mut reader, &mut res);
    print!("{}", res);
}
