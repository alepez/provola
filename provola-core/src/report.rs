use serde::{Deserialize, Serialize};

type Timestamp = String; // TODO
type Duration = String; // TODO
type Size = usize;

struct Report {
    disabled: Option<Size>,
    errors: Option<Size>,
    failures: Option<Size>,
    name: Option<String>,
    tests: Option<Size>,
    testsuites: Vec<TestSuite>,
    time: Option<Duration>,
    timestamp: Option<Timestamp>,
}

struct TestSuite {
    // TODO properties
    // TODO system-err
    // TODO system-out
    disabled: Option<Size>,
    errors: Option<Size>,
    failures: Option<Size>,
    hostname: Option<String>,
    id: Option<String>,
    name: String,
    package: Option<String>,
    skipped: Option<Size>,
    testcases: Vec<TestCase>,
    tests: Size,
    time: Option<Duration>,
    timestamp: Option<Time>,
}

struct TestCase {
    // TODO error
    // TODO failure
    // TODO skipped
    // TODO system-err
    // TODO system-out
    assertions: Option<String>,
    classname: Option<String>,
    name: String,
    status: Option<String>,
    time: Option<Duration>,
}
