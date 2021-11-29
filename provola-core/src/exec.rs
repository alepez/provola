use std::convert::TryFrom;
use std::path::PathBuf;

use crate::{build::gen_executable, Language, Source};

#[derive(Debug)]
pub enum Executable {
    Simple(PathBuf),
    Multiple(Vec<String>),
}

impl From<PathBuf> for Executable {
    fn from(path: PathBuf) -> Self {
        Executable::Simple(path)
    }
}

impl TryFrom<(Language, &Source)> for Executable {
    type Error = crate::Error;

    fn try_from(x: (Language, &Source)) -> Result<Self, Self::Error> {
        let (lang, source) = x;
        let exe = gen_executable(lang, source)?;
        Ok(exe)
    }
}

impl Into<Vec<String>> for &Executable {
    fn into(self) -> Vec<String> {
        match &self {
            Executable::Simple(path) => vec![path.as_os_str().to_str().unwrap().to_string()],
            &Executable::Multiple(x) => x.clone(),
        }
    }
}

impl Executable {
    pub fn interpreted(interpreter: String, source: &Source) -> Self {
        let source = source.0.as_os_str().to_str().unwrap().to_string();
        let argv = vec![interpreter, source];
        Executable::Multiple(argv)
    }
}
