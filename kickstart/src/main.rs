#![deny(clippy::all)]
#![allow(dead_code)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]

mod year2013;
mod year2020;

use std::fmt::Write;
use std::io::*;

#[derive(Debug)]
pub enum RustGymError {
    Fmt(std::fmt::Error),
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
}

impl From<std::fmt::Error> for RustGymError {
    fn from(err: std::fmt::Error) -> Self {
        RustGymError::Fmt(err)
    }
}

impl From<std::io::Error> for RustGymError {
    fn from(err: std::io::Error) -> Self {
        RustGymError::Io(err)
    }
}

impl From<std::num::ParseIntError> for RustGymError {
    fn from(err: std::num::ParseIntError) -> Self {
        RustGymError::ParseInt(err)
    }
}

pub type RustGymResult = std::result::Result<(), RustGymError>;

fn main() -> RustGymResult {
    let mut res = "".to_string();
    solve(&mut BufReader::new(stdin()), &mut res)?;
    print!("{}", res);
    Ok(())
}

fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) -> RustGymResult {
    let mut it = reader.lines().map(|l| l.unwrap());
    let line = it.next().unwrap();
    write!(writer, "{}", line)?;
    Ok(())
}
