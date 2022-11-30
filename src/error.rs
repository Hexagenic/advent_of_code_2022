#[derive(Debug)]
pub enum Error {
    ParseIntError,
    IoError(std::io::Error),
}

impl From<std::num::ParseIntError> for Error {
    fn from(_error: std::num::ParseIntError) -> Self {
        Error::ParseIntError
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}
