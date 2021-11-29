mod rust;
mod haskell;
mod common;

use std::path::PathBuf;

use crate::actions::Source;
use crate::errors::Error;
use crate::lang::Language;

pub fn gen_executable(lang: Language, source: &Source) -> Result<PathBuf, Error> {
    match lang {
        Language::Rust => rust::build(source),
        Language::Haskell => haskell::build(source),
        _ => Err(Error::LangNotSupported(lang.to_string()))
    }
}