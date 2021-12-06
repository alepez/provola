use provola_core::Report as CoreReport;
use provola_core::TestCase as CoreTestCase;
use provola_core::TestSuite as CoreTestSuite;
use serde::{Deserialize, Serialize};

type Duration = String;
type Timestamp = provola_core::report::Timestamp;

/// Parse "1s" as 1 second, "10s" as 10 seconds...
fn parse_duration(s: &str) -> Option<std::time::Duration> {
    if s.len() < 2 {
        return None;
    }

    let s = &s[0..(s.len() - 1)];
    let secs: f32 = s.parse().ok()?;
    let millis: u64 = (secs * 1000.0).round() as u64;
    let duration = std::time::Duration::from_millis(millis);
    Some(duration)
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UnitTest {
    tests: usize,
    failures: usize,
    disabled: usize,
    errors: usize,
    timestamp: Timestamp,
    time: Duration,
    name: String,
    testsuites: Vec<TestCase>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TestCase {
    name: String,
    tests: usize,
    failures: usize,
    disabled: usize,
    errors: usize,
    time: Duration,
    testsuite: Vec<TestInfo>,
    timestamp: Timestamp,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Status {
    #[serde(rename = "RUN")]
    Run,
    #[serde(rename = "NOTRUN")]
    NotRun,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Failure {
    failure: String,
    #[serde(rename = "type")]
    ttype: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TestInfo {
    name: String,
    status: Status,
    time: Duration,
    classname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure: Option<Failure>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    failures: Vec<Failure>,
    result: String,
    timestamp: Timestamp,
}

impl From<UnitTest> for CoreReport {
    fn from(x: UnitTest) -> Self {
        CoreReport {
            disabled: Some(x.disabled),
            errors: Some(x.errors),
            failures: Some(x.failures),
            name: Some(x.name),
            tests: Some(x.tests),
            time: parse_duration(&x.time),
            timestamp: Some(x.timestamp),
            testsuites: x.testsuites.into_iter().map(|x| x.into()).collect(),
            ..Default::default()
        }
    }
}

impl From<TestCase> for CoreTestSuite {
    fn from(x: TestCase) -> Self {
        CoreTestSuite {
            name: x.name,
            tests: x.tests,
            disabled: Some(x.disabled),
            errors: Some(x.errors),
            failures: Some(x.failures),
            testcases: x.testsuite.into_iter().map(|x| x.into()).collect(),
            time: parse_duration(&x.time),
            timestamp: Some(x.timestamp),
            ..Default::default()
        }
    }
}

impl From<TestInfo> for CoreTestCase {
    fn from(x: TestInfo) -> Self {
        CoreTestCase {
            name: x.name,
            status: None, // TODO Convert to string, but to which format? Some(x.status),
            time: parse_duration(&x.time),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn parse_json_report() {
        let path = "examples/data/test_report.json";
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let _u: UnitTest = serde_json::from_reader(reader).unwrap();
    }

    #[test]
    fn parse_duration_from_str() {
        assert_eq!(parse_duration(""), None);
        assert_eq!(parse_duration("s"), None);
        assert_eq!(parse_duration("ss"), None);
        assert_eq!(
            parse_duration("1s"),
            Some(std::time::Duration::from_secs(1))
        );
    }
}
