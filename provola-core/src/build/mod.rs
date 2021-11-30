mod ada;
mod bash;
mod c;
mod caml;
mod clojure;
mod cpp;
mod csharp;
mod dart;
mod elixir;
mod erlang;
mod fsharp;
mod go;
mod groovy;
mod haskell;
mod javascript;
mod java;
mod kotlin;
mod lisp;
mod objectivec;
mod php;
mod python;
mod r;
mod ruby;
mod rust;
mod scala;
mod swift;
mod typescript;
mod vba;

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
        Language::Ada => ada::build(source),
        Language::Caml => caml::build(source),
        Language::CSharp => csharp::build(source),
        Language::Clojure => clojure::build(source),
        Language::Dart => dart::build(source),
        Language::Elixir => elixir::build(source),
        Language::Erlang => erlang::build(source),
        Language::FSharp => fsharp::build(source),
        Language::Go => go::build(source),
        Language::Groovy => groovy::build(source),
        Language::Java => java::build(source),
        Language::JavaScript => javascript::build(source),
        Language::Kotlin => kotlin::build(source),
        Language::Lisp => lisp::build(source),
        Language::ObjectiveC => objectivec::build(source),
        Language::PHP => php::build(source),
        Language::Python => python::build(source),
        Language::R => r::build(source),
        Language::Ruby => ruby::build(source),
        Language::Scala => scala::build(source),
        Language::Swift => swift::build(source),
        Language::TypeScript => typescript::build(source),
        Language::VBA => vba::build(source),
    }
}

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

pub type CommandGenerator = fn(&Path, &Source) -> Command;

pub fn build(source: &Source, gen_command: CommandGenerator) -> Result<Executable, Error> {
    let exec = PathBuf::from("./tmp.exe");
    let mut cmd = gen_command(&exec, source);
    log::info!("Running {:?}", cmd);
    let Output { status, stderr, .. } = cmd.output()?;

    let stderr = String::from_utf8(stderr).unwrap_or_default();

    if status.success() {
        Ok(exec.into())
    } else {
        let msg = format!("Cannot build executable with command {:?}: {}", cmd, stderr);
        log::error!("{}", &msg);
        Err(Error::BuildFailed(msg))
    }
}

pub(crate) fn interpret(source: &Source, interpreter: impl ToString) -> Result<Executable, Error> {
    Ok(Executable::interpreted(interpreter.to_string(), source))
}
