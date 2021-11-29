#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no test result available")]
    NoResult,
    #[error("executable not available")]
    NoExecutable,
    #[error("cannot build: {0}")]
    BuildFailed(String),
    #[error("i/o error")]
    IoError(#[from] std::io::Error),
}
