use provola_core::Report as CoreReport;
use serde::{Deserialize, Serialize};

type Duration = String;
type Timestamp = String;

fn parse_duration(s: &str) -> Option<std::time::Duration> {
    todo!()
}

fn parse_timestamp(s: &str) -> Option<std::time::SystemTime> {
    todo!()
}

#[derive(Serialize, Deserialize, Debug)]
struct UnitTest {
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
struct TestCase {
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
enum Status {
    #[serde(rename = "RUN")]
    Run,
    #[serde(rename = "NOTRUN")]
    NotRun,
}

#[derive(Serialize, Deserialize, Debug)]
struct Failure {
    failure: String,
    #[serde(rename = "type")]
    ttype: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestInfo {
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
        let mut y = CoreReport::default();

        y.disabled = Some(x.disabled);
        y.errors = Some(x.errors);
        y.failures = Some(x.failures);
        y.name = Some(x.name);
        y.tests = Some(x.tests);
        y.time = parse_duration(&x.time);
        y.timestamp = parse_timestamp(&x.timestamp);

        y
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufReader, BufWriter},
    };

    use super::*;

    #[test]
    fn parse_json_report() {
        let path = "examples/data/test_report.json";
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let u: UnitTest = serde_json::from_reader(reader).unwrap();
        // let path_out = "examples/data/test_report_out.json";
        // let file_out = File::create(path_out).unwrap();
        // let writer = BufWriter::new(file_out);
        // serde_json::to_writer(writer, &u).unwrap();
    }
}
