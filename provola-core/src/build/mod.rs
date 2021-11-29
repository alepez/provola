mod rust;
mod haskell;

use std::path::PathBuf;

use crate::actions::Source;
use crate::errors::Error;
use crate::lang::Language;

pub fn gen_executable(lang: Language, source: &Source) -> Result<PathBuf, Error> {
    match lang {
        Language::Rust => rust::build(source),
        Language::Haskell => haskell::build(source),
        _ => todo!(),
    }
}
