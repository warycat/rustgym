mod _30_hello_world;

use std::io::*;

fn solve(_: &mut dyn BufRead) -> Result<String> {
    Ok("".to_string())
}

fn main() {
    print!("{}", solve(&mut BufReader::new(stdin())).unwrap());
}
