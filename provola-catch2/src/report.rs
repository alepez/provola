use provola_core::Failure as CoreFailure;
use provola_core::Report as CoreReport;
use provola_core::TestCase as CoreTestCase;
use provola_core::TestSuite as CoreTestSuite;
use serde::{Deserialize, Serialize};

type Duration = String;

pub type Name = String;
pub type Status = String;
pub type ClassName = String;
pub type FailureType = String;
pub type Message = String;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Report {
    #[serde(rename = "testsuite")]
    pub testsuites: Vec<TestSuite>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TestSuite {
    // // TODO system-err
    // // TODO system-out
    #[serde(rename = "testcase", default)]
    pub testcases: Vec<TestCase>,
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
        // TODO
        CoreReport {
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn parse_xml_report() {
        let path = "examples/data/test_report.xml";
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let u: Report = serde_xml_rs::from_reader(reader).unwrap();
        insta::assert_debug_snapshot!(&u);
    }
}
