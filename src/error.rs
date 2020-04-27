use std::{error, fmt, result};

#[derive(Debug)]
pub enum Error {

}

impl error::Error for Error {

}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

pub type Result<T> = result::Result<T, Error>;
