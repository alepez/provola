use provola_core::test_runners::AvailableTests;
use provola_core::{Error, Executable, Report};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use subprocess::Popen;
use subprocess::PopenConfig;
use subprocess::Redirection;

mod report;

fn add_list_argv(mut argv: Vec<String>) -> Vec<String> {
    argv.push("--gtest_list_tests".to_string());
    argv.push("--gtest_color=no".to_string());
    argv
}

fn add_run_argv(mut argv: Vec<String>, report_path: &str) -> Vec<String> {
    argv.push(format!("--gtest_output=json:{}", report_path));
    argv.push("--gtest_color=no".to_string());
    argv
}

fn run_exec_with_argv(argv: Vec<String>) -> Result<String, Error> {
    let mut p = Popen::create(
        &argv,
        PopenConfig {
            stdin: Redirection::None,
            stdout: Redirection::Pipe,
            stderr: Redirection::Pipe,
            ..Default::default()
        },
    )?;

    let (out, _err) = p.communicate(None)?;

    // TODO Timeout from configuration
    let timeout = Duration::from_secs(3600);

    if let Some(_exit_status) = p.wait_timeout(timeout)? {
        log::debug!("Test done");
    } else {
        log::warn!("Terminate subprocess");
        p.terminate()?;
    }

    Ok(out.unwrap_or_default())
}

fn extract_test_suite_name(s: &str) -> String {
    s.chars().take_while(|&x| x != '.').collect()
}

fn extract_test_case_name(s: &str) -> String {
    s.chars().skip(2).collect()
}

fn parse_available_tests(s: &str) -> Result<AvailableTests, Error> {
    let lines = s.lines().skip(1);

    let mut test_suite: Option<String> = None;
    let mut tests = AvailableTests::default();

    for line in lines {
        // At least 2 chars, one for the name, one for dot or space
        // "X." is the shortest test suite name
        // "  X" is the shortest test case name
        match line.chars().nth(0) {
            Some(' ') => {
                if let Some(test_suite) = &test_suite {
                    tests.push(test_suite.clone(), extract_test_case_name(line));
                }
            }
            Some(_) => {
                test_suite = Some(extract_test_suite_name(line));
            }
            None => {
                // invalid
            }
        }
    }

    Ok(tests)
}

fn generate_available_tests(executable: &Executable) -> Result<AvailableTests, Error> {
    let argv = add_list_argv(executable.into());
    let out = run_exec_with_argv(argv)?;
    parse_available_tests(&out)
}

fn generate_report(executable: &Executable) -> Result<Report, Error> {
    let report_path = "googletest_report.json";
    let argv = add_run_argv(executable.into(), report_path);
    run_exec_with_argv(argv)?;

    let file = File::open(report_path).unwrap();
    let reader = BufReader::new(file);
    let gtest_rep: report::UnitTest = serde_json::from_reader(reader).unwrap();
    let core_rep = Report::from(gtest_rep);
    Ok(core_rep)
}

pub struct TestRunner {
    executable: Executable,
}

impl From<Executable> for TestRunner {
    fn from(executable: Executable) -> Self {
        TestRunner { executable }
    }
}

impl provola_core::test_runners::TestRunner for TestRunner {
    fn run(&self) -> Result<provola_core::TestResult, provola_core::Error> {
        let report = generate_report(&self.executable)?;
        let result = report.into();
        Ok(result)
    }

    fn list(&self) -> Result<AvailableTests, Error> {
        generate_available_tests(&self.executable)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    // Ignored because example must be built first
    #[ignore]
    #[test]
    fn run_valid_executable() {
        let path = PathBuf::from("./examples/data/build/example");
        let exec = Executable::from(path);
        assert!(generate_report(&exec).is_ok());
    }

    // Ignored because example must be built first
    #[ignore]
    #[test]
    fn from_valid_executable() {
        let path = PathBuf::from("./examples/data/build/example");
        let exec = Executable::from(path);
        let tr = TestRunner::from(exec);
        let tr: Box<dyn provola_core::test_runners::TestRunner> = Box::new(tr);
        assert!(tr.run().is_ok());
    }

    // Ignored because example must be built first
    #[ignore]
    #[test]
    fn generate_available_tests_from_valid_executable() {
        let path = PathBuf::from("./examples/data/build/example");
        let exec = Executable::from(path);
        let list = generate_available_tests(&exec).unwrap();
        assert_eq!(list.len(), 4);
    }

    #[test]
    fn parse_gtest_list_tests_output() {
        let s = r#"Running main() from provola-googletest/examples/data/googletest/googletest/src/gtest_main.cc
Foo.
  Foo1
  Foo2
Bar.
  Bar1
  Bar2"#;
        let list = parse_available_tests(s).unwrap();
        insta::assert_debug_snapshot!(&list);
    }
}
