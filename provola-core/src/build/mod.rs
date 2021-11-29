mod c;
mod common;
mod cpp;
mod haskell;
mod rust;

use std::path::PathBuf;

use crate::actions::Source;
use crate::errors::Error;
use crate::lang::Language;

pub fn gen_executable(lang: Language, source: &Source) -> Result<PathBuf, Error> {
    match lang {
        Language::C => c::build(source),
        Language::CPlusPlus => cpp::build(source),
        Language::Haskell => haskell::build(source),
        Language::Rust => rust::build(source),
        _ => Err(Error::LangNotSupported(lang.to_string())),
    }
}
