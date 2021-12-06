use serde::{Deserialize, Serialize};

type Timestamp = std::time::SystemTime;
type Duration = std::time::Duration;
type Count = usize;
type Name = String;
type Status = String;
type Hostname = String;
type Id = String;
type Package = String;
type ClassName = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    pub disabled: Option<Count>,
    pub errors: Option<Count>,
    pub failures: Option<Count>,
    pub name: Option<Name>,
    pub tests: Option<Count>,
    pub testsuites: Vec<TestSuite>,
    pub time: Option<Duration>,
    pub timestamp: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestSuite {
    // TODO properties
    // TODO system-err
    // TODO system-out
    pub disabled: Option<Count>,
    pub errors: Option<Count>,
    pub failures: Option<Count>,
    pub hostname: Option<Hostname>,
    pub id: Option<Id>,
    pub name: Name,
    pub package: Option<Package>,
    pub skipped: Option<Count>,
    pub testcases: Vec<TestCase>,
    pub tests: Count,
    pub time: Option<Duration>,
    pub timestamp: Option<Duration>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestCase {
    // TODO error
    // TODO assertions
    // TODO failure
    // TODO skipped
    // TODO system-err
    // TODO system-out
    pub classname: Option<ClassName>,
    pub name: Name,
    pub status: Option<Status>,
    pub time: Option<Duration>,
}
