use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct CodeLine(u32);

impl From<u32> for CodeLine {
    fn from(n: u32) -> Self {
        Self(n)
    }
}

#[derive(Debug, Clone)]
pub struct CodeReference {
    path: PathBuf,
    line: CodeLine,
}

impl CodeReference {
    pub fn new(path: impl Into<PathBuf>, line: impl Into<CodeLine>) -> Self {
        Self {
            path: path.into(),
            line: line.into(),
        }
    }
}

impl Display for CodeReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}:{}]",
            self.path.as_os_str().to_str().unwrap(),
            self.line.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_reference_has_path_and_line() {
        let cr = CodeReference::new("/foo/bar.rs", 32);
        assert_eq!(CodeLine(32), cr.line)
    }

    #[test]
    fn code_reference_can_be_displayed() {
        let cr = CodeReference::new("/foo/bar.rs", 32);
        assert_eq!("[/foo/bar.rs:32]", cr.to_string())
    }
}

