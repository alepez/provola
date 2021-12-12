use crate::TestResult;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("i/o error")]
    IoError(#[from] std::io::Error),
}

pub trait Reporter {
    fn report(&self, result: TestResult) -> Result<(), Error>;
}
