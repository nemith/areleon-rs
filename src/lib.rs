use format_serde_error::SerdeError;
use thiserror::Error;

pub mod def;
pub mod generator;
pub mod policy;

#[derive(Error, Debug)]
pub enum Error {
    #[error("policy file syntax error")]
    InputFileError(#[from] SerdeError),
}

pub type Result<T> = std::result::Result<T, Error>;
