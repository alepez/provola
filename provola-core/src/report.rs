use serde::{Deserialize, Serialize};

use crate::test::xunit::FullyQualifiedTestCaseId;

pub type Timestamp = chrono::DateTime<chrono::Utc>;
pub type Duration = std::time::Duration;
pub type Count = usize;
pub type Name = String;
pub type Hostname = String;
pub type Id = String;
pub type Package = String;
pub type ClassName = String;
pub type FailureType = String;
pub type Message = String;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CoreStatus {
    Unknown,
    Pass,
    Fail,
    Ignored,
    Skipped,
}

impl Default for CoreStatus {
    fn default() -> Self {
        CoreStatus::Unknown
    }
}

impl From<Option<bool>> for CoreStatus {
    fn from(ok: Option<bool>) -> Self {
        match ok {
            None => CoreStatus::Unknown,
            Some(true) => CoreStatus::Pass,
            Some(false) => CoreStatus::Fail,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CoreReport {
    pub disabled: Option<Count>,
    pub errors: Option<Count>,
    /// The total number of rule violations
    pub failures: Option<Count>,
    /// The label of the scan
    pub name: Option<Name>,
    /// The total number of rules that were applied
    pub tests: Option<Count>,
    pub testsuites: Vec<CoreTestSuite>,
    /// The time that was required to process all the rules
    pub time: Option<Duration>,
    pub timestamp: Option<Timestamp>,
}

impl CoreReport {
    pub fn sort(&mut self) {
        self.testsuites.sort_by(|x, y| x.name.cmp(&y.name));
        for test_suite in &mut self.testsuites {
            test_suite.testcases.sort_by(|x, y| x.name.cmp(&y.name));
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CoreTestSuite {
    // TODO properties
    // TODO system-err
    // TODO system-out
    pub disabled: Option<Count>,
    pub errors: Option<Count>,
    pub failures: Option<Count>,
    pub hostname: Option<Hostname>,
    pub id: Option<Id>,
    /// The label of the provider
    pub name: Name,
    pub package: Option<Package>,
    pub skipped: Option<Count>,
    pub testcases: Vec<CoreTestCase>,
    /// The number of rules in the provider that were applied
    pub tests: Count,
    /// The time that was required to process the rules in the provider
    pub time: Option<Duration>,
    pub timestamp: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CoreTestCase {
    // TODO fqtc should not be an Option
    pub fqtc: Option<FullyQualifiedTestCaseId>,
    // TODO error
    // TODO assertions
    // TODO skipped
    // TODO system-err
    // TODO system-out
    pub classname: Option<ClassName>,
    /// The label of the rule
    pub name: Name,
    pub status: CoreStatus,
    /// The time that was required to process all the applications of this rule
    pub time: Option<Duration>,
    pub failures: Vec<CoreFailure>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CoreFailure {
    pub ttype: FailureType,
    pub message: Message,
}
