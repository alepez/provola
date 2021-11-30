mod bash;
mod c;
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
        Language::Ada => todo!(),
        Language::Caml => todo!(),
        Language::CSharp => todo!(),
        Language::Clojure => todo!(),
        Language::Dart => todo!(),
        Language::Elixir => todo!(),
        Language::Erlang => todo!(),
        Language::FSharp => todo!(),
        Language::Go => todo!(),
        Language::Groovy => todo!(),
        Language::Java => todo!(),
        Language::JavaScript => todo!(),
        Language::Kotlin => todo!(),
        Language::Lisp => todo!(),
        Language::ObjectiveC => todo!(),
        Language::PHP => todo!(),
        Language::Python => todo!(),
        Language::R => todo!(),
        Language::Ruby => todo!(),
        Language::Scala => todo!(),
        Language::Swift => todo!(),
        Language::TypeScript => todo!(),
        Language::VBA => todo!(),
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
