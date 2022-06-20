use std::path::PathBuf;

#[derive(Debug)]
pub struct CodeReference {
    pub path: PathBuf,
    pub line: usize,
}