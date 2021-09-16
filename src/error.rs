use thiserror::Error;

#[derive(Error, Debug)]
pub enum DidrteError {
    #[error("Some tests have failed")]
    Failed,

    #[error("Unable to list tests directory {d}")]
    ListTestDir {
        d: std::path::PathBuf,
        source: std::io::Error,
    },
}

pub type DidrteResult<T> = std::result::Result<T, DidrteError>;
