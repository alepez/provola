#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no test result available")]
    NoResult,
    #[error("no executable available")]
    NoExecutable,
    #[error("cannot read input file")]
    NoInputFile(#[from] std::io::Error),
}
