use std::{convert::TryFrom, path::PathBuf};

use crate::{build::gen_executable, Language, Source};

#[derive(Debug)]
pub struct Executable(PathBuf);

impl TryFrom<(Language, &Source)> for Executable {
    type Error = Box<dyn std::error::Error>;

    fn try_from(x: (Language, &Source)) -> Result<Self, Self::Error> {
        let (lang, source) = x;
        let path = gen_executable(lang, source)?;
        Ok(Executable(path))
    }
}

impl Executable {
    pub fn path(&self) -> &PathBuf {
        &self.0
    }
}
