mod rust;

use std::path::PathBuf;

use crate::actions::Source;
use crate::errors::Error;
use crate::lang::Language;

pub fn gen_executable(lang: Language, source: &Source) -> Result<PathBuf, Error> {
    match lang {
        Language::Rust => rust::build(source),
        _ => todo!(),
    }
}
