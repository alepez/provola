use provola_core::test_runners::{AvailableTests, Only, TestRunnerOpt};
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
        match line.chars().next() {
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

fn generate_report(executable: &Executable, test_filter: &TestFilter) -> Result<Report, Error> {
    let report_path = "googletest_report.json";
    let executable = executable.into();

    let mut argv = add_run_argv(executable, report_path);

    if let Some(test_filter_s) = &test_filter.0 {
        argv.push(format!("--gtest_filter={}", test_filter_s));
    }

    run_exec_with_argv(argv)?;

    let file = File::open(report_path).unwrap();
    let reader = BufReader::new(file);
    let gtest_rep: report::UnitTest = serde_json::from_reader(reader).unwrap();
    let core_rep = Report::from(gtest_rep);
    Ok(core_rep)
}

pub struct TestRunner {
    executable: Executable,
    available_tests: AvailableTests,
}

struct TestFilter(Option<String>);

impl Default for TestFilter {
    fn default() -> Self {
        TestFilter(None)
    }
}

fn make_test_filter(opt: &TestRunnerOpt, tests: &AvailableTests) -> Result<TestFilter, Error> {
    let fqtn = match opt.only {
        Only::SingleByIndex(index) => tests.get(index),
        Only::All => None,
    };

    let s = fqtn.map(|x| format!("{}.{}", x.test_suite.0, x.test_case.0));

    Ok(TestFilter(s))
}

impl TestRunner {
    fn generate_report(&self, opt: &TestRunnerOpt) -> Result<Report, Error> {
        let test_filter = make_test_filter(opt, &self.available_tests);
        generate_report(&self.executable, &test_filter?)
    }
}

impl From<Executable> for TestRunner {
    fn from(executable: Executable) -> Self {
        // TODO Fix unwrap
        let available_tests = generate_available_tests(&executable).unwrap();
        TestRunner {
            executable,
            available_tests,
        }
    }
}

impl provola_core::test_runners::TestRunner for TestRunner {
    fn run(&self, opt: &TestRunnerOpt) -> Result<provola_core::TestResult, provola_core::Error> {
        let report = self.generate_report(&opt)?;
        let result = report.into();
        Ok(result)
    }

    fn list(&self, _opt: &TestRunnerOpt) -> Result<AvailableTests, Error> {
        generate_available_tests(&self.executable)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn make_exec() -> Executable {
        let path = PathBuf::from("./examples/data/build/example");
        Executable::from(path)
    }

    // Ignored because example must be built first
    #[ignore]
    #[test]
    fn run_valid_executable() {
        let exec = make_exec();
        let test_filter = TestFilter::default();
        assert!(generate_report(&exec, &test_filter).is_ok());
    }

    // Ignored because example must be built first
    #[ignore]
    #[test]
    fn from_valid_executable() {
        let exec = make_exec();
        let tr = TestRunner::from(exec);
        let tr: Box<dyn provola_core::test_runners::TestRunner> = Box::new(tr);
        let tr_opt = TestRunnerOpt::default();
        assert!(tr.run(&tr_opt).is_ok());
    }

    // Ignored because example must be built first
    #[ignore]
    #[test]
    fn generate_available_tests_from_valid_executable() {
        let exec = make_exec();
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
