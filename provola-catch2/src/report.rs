use provola_core::CoreFailure;
use provola_core::CoreReport;
use provola_core::CoreTestCase;
use provola_core::CoreTestSuite;
use serde::{Deserialize, Serialize};

type Duration = String;

pub type Name = String;
pub type Status = String;
pub type ClassName = String;
pub type FailureType = String;
pub type Message = String;

fn parse_duration(s: &str) -> Option<std::time::Duration> {
    s.parse().ok().map(std::time::Duration::from_secs)
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Report {
    #[serde(rename = "testsuite")]
    pub testsuites: Vec<TestSuite>,
}

impl Report {
    fn failures_count(&self) -> usize {
        self.testsuites.iter().map(|x| x.failures_count()).sum()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TestSuite {
    // // TODO system-err
    // // TODO system-out
    #[serde(rename = "testcase", default)]
    pub testcases: Vec<TestCase>,
}

impl TestSuite {
    fn failures_count(&self) -> usize {
        self.testcases.iter().map(|x| x.failures.len()).sum()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TestCase {
    pub classname: ClassName,
    pub name: Name,
    pub status: Status,
    pub time: Duration,
    #[serde(rename = "failure", default)]
    pub failures: Vec<Failure>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Failure {
    #[serde(rename = "type")]
    pub ttype: FailureType,
    pub message: Message,
}

impl From<Report> for CoreReport {
    fn from(x: Report) -> Self {
        let failures = Some(x.failures_count());
        CoreReport {
            testsuites: x.testsuites.into_iter().map(|x| x.into()).collect(),
            failures,
            ..Default::default()
        }
    }
}

impl From<TestSuite> for CoreTestSuite {
    fn from(x: TestSuite) -> Self {
        let failures = Some(x.failures_count());
        CoreTestSuite {
            testcases: x.testcases.into_iter().map(|x| x.into()).collect(),
            failures,
            ..Default::default()
        }
    }
}

impl From<TestCase> for CoreTestCase {
    fn from(x: TestCase) -> Self {
        CoreTestCase {
            name: x.name,
            classname: Some(x.classname),
            status: Some(x.status),
            time: parse_duration(&x.time),
            failures: x.failures.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<Failure> for CoreFailure {
    fn from(x: Failure) -> Self {
        CoreFailure {
            message: x.message,
            ttype: x.ttype,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    fn read_example_file() -> Report {
        let path = "examples/data/test_report.xml";
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_xml_rs::from_reader(reader).unwrap()
    }

    #[test]
    fn parse_xml_report() {
        let report = read_example_file();
        insta::assert_debug_snapshot!(&report);
    }

    #[test]
    fn convert_to_core_report() {
        let report = read_example_file();
        let report = CoreReport::from(report);
        insta::assert_debug_snapshot!(&report);
    }
}
