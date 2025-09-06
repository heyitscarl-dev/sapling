use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid argument count: expected {expected} but got {got}")]
    ArgumentCount {
        expected: u8,
        got: u8
    },

    #[error("Invalid argument name: {0} is not a recognized argument.")]
    ArgumentName(String),
}

pub type Result<T> = std::result::Result<T, Error>;
