use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CodeReference {
    pub path: PathBuf,
    pub line: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_reference_has_path_and_line() {
        let cr = CodeReference {
            path: PathBuf::from("/foo/bar.rs"),
            line: 32,
        };
        assert_eq!(32, cr.line)
    }
}