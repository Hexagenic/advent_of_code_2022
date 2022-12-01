#[derive(Debug)]
pub enum Error {
    ParseIntError,
    IoError(std::io::Error),
}

impl From<std::num::ParseIntError> for Error {
    fn from(_error: std::num::ParseIntError) -> Self {
        Self::ParseIntError
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
