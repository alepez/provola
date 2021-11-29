mod bash;
mod c;
mod common;
mod cpp;
mod haskell;
mod rust;

use crate::actions::Source;
use crate::errors::Error;
use crate::lang::Language;
use crate::Executable;

pub fn gen_executable(lang: Language, source: &Source) -> Result<Executable, Error> {
    match lang {
        Language::Bash => bash::build(source),
        Language::C => c::build(source),
        Language::CPlusPlus => cpp::build(source),
        Language::Haskell => haskell::build(source),
        Language::Rust => rust::build(source),
        _ => Err(Error::LangNotSupported(lang.to_string())),
    }
}
