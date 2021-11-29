use std::{fmt::Display, path::Path, str::FromStr};

#[derive(Debug, Copy, Clone)]
pub enum Language {
    Ada,
    Bash,
    C,
    Caml,
    CPlusPlus,
    CSharp,
    Clojure,
    Dart,
    Elixir,
    Erlang,
    FSharp,
    Go,
    Groovy,
    Haskell,
    Java,
    JavaScript,
    Kotlin,
    Lisp,
    ObjectiveC,
    PHP,
    Python,
    R,
    Ruby,
    Rust,
    Scala,
    Swift,
    TypeScript,
    VBA,
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Language::*;

        let s = String::from(s);
        let s = s.to_lowercase();
        let s = s.as_str();

        match s {
            "ada" => Ok(Ada),
            "bash" => Ok(Bash),
            "c" => Ok(C),
            "caml" => Ok(Caml),
            "cpp" | "c++" | "cxx" | "cplusplus" => Ok(CPlusPlus),
            "c#" | "csharp" => Ok(CSharp),
            "clojure" => Ok(Clojure),
            "dart" => Ok(Dart),
            "elixir" => Ok(Elixir),
            "erlang" => Ok(Erlang),
            "f#" | "fsharp" => Ok(FSharp),
            "go" => Ok(Go),
            "groovy" => Ok(Groovy),
            "haskell" => Ok(Haskell),
            "java" => Ok(Java),
            "javascript" => Ok(JavaScript),
            "kotlin" => Ok(Kotlin),
            "lisp" => Ok(Lisp),
            "objectivec" => Ok(ObjectiveC),
            "php" => Ok(PHP),
            "python" => Ok(Python),
            "r" => Ok(R),
            "ruby" => Ok(Ruby),
            "rust" => Ok(Rust),
            "scala" => Ok(Scala),
            "swift" => Ok(Swift),
            "typeScript" => Ok(TypeScript),
            "vba" => Ok(VBA),
            _ => Err("Invalid language".to_string()),
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            Language::Ada => "Ada",
            Language::Bash => "bash",
            Language::C => "C",
            Language::Caml => "caml",
            Language::CPlusPlus => "C++",
            Language::CSharp => "C#",
            Language::Clojure => "Clojure",
            Language::Dart => "dart",
            Language::Elixir => "elixir",
            Language::Erlang => "erlang",
            Language::FSharp => "f#",
            Language::Go => "Go",
            Language::Groovy => "Groovy",
            Language::Haskell => "Haskell",
            Language::Java => "Java",
            Language::JavaScript => "JavaScript",
            Language::Kotlin => "Kotlin",
            Language::Lisp => "Lisp",
            Language::ObjectiveC => "ObjectiveC",
            Language::PHP => "PHP",
            Language::Python => "Python",
            Language::R => "R",
            Language::Ruby => "Ruby",
            Language::Rust => "Rust",
            Language::Scala => "Scala",
            Language::Swift => "Swift",
            Language::TypeScript => "TypeScript",
            Language::VBA => "VBA",
        };

        write!(f, "{}", s)
    }
}

impl Language {
    pub fn from_source(source: &Path) -> Option<Language> {
        source
            .extension()
            .and_then(|x| x.to_str())
            .and_then(|ext| match ext {
                "sh" => Some(Language::Bash),
                "c" => Some(Language::C),
                "cpp" | "cxx" | "c++" => Some(Language::CPlusPlus),
                "hs" => Some(Language::Haskell),
                _ => None,
            })
    }
}
