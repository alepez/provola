use std::path::PathBuf;

use crate::actions::Source;
use crate::lang::Language;

pub fn gen_executable(
    lang: Language,
    source: &Source,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    match lang {
        Language::Rust => build_rust(source),
        _ => todo!(),
    }
}

fn make_build_rust_command(exec: &PathBuf, source: &Source) -> std::process::Command {
    use std::process::Command;
    let mut cmd = Command::new("rustc");
    cmd.arg(&source.0).arg("-o").arg(exec);
    cmd
}

fn build_rust(source: &Source) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let exec = PathBuf::from("./tmp.exe");
    let mut cmd = make_build_rust_command(&exec, source);
    log::info!("Running {:?}", cmd);
    cmd.output()?;
    Ok(exec)
}
