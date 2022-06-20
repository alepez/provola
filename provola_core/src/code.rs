use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CodeReference {
    pub path: PathBuf,
    pub line: usize,
}

impl Display for CodeReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.path.as_os_str().to_str().unwrap(), self.line)
    }
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

    #[test]
    fn code_reference_can_be_displayed() {
        let cr = CodeReference {
            path: PathBuf::from("/foo/bar.rs"),
            line: 32,
        };
        assert_eq!("/foo/bar.rs:32", cr.to_string())
    }
}