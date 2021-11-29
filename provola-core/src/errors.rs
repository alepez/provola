#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no test result available")]
    NoResult,
    #[error("executable not available")]
    NoExecutable,
    #[error("cannot build: {0}")]
    BuildFailed(String),
    #[error("cannot read input file")]
    NoInputFile(#[from] std::io::Error),
}
