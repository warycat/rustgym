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
