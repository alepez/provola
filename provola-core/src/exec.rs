use std::{convert::TryFrom, path::Path};
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

fn path_to_string(path: &Path) -> String {
    path.as_os_str().to_str().unwrap().to_string()
}

impl Into<Vec<String>> for &Executable {
    fn into(self) -> Vec<String> {
        match &self {
            Executable::Simple(path) => vec![path_to_string(path)],
            &Executable::Multiple(x) => x.clone(),
        }
    }
}

impl Executable {
    pub fn interpreted(interpreter: String, source: &Source) -> Self {
        let source = path_to_string(&source.0);
        let argv = vec![interpreter, source];
        Executable::Multiple(argv)
    }
}
