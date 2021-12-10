use provola_core::test_runners::TestRunner;
use provola_core::{Error, Executable};
use strum_macros::{Display, EnumString};

pub fn make_test_runner(info: TestRunnerInfo) -> Result<Box<dyn TestRunner>, Error> {
    let test_runner_type = info.trt;
    match test_runner_type {
        #[cfg(feature = "googletest")]
        TestRunnerType::GoogleTest => {
            Ok(provola_googletest::TestRunner::from_executable(info.exec))
        }
    }
}

#[derive(Debug, EnumString, Clone, Copy, Display)]
pub enum TestRunnerType {
    #[cfg(feature = "googletest")]
    GoogleTest,
}

#[derive(Debug)]
pub struct TestRunnerInfo {
    pub exec: Executable,
    pub trt: TestRunnerType,
}

impl TestRunnerInfo {
    pub fn new(exec: Executable, trt: TestRunnerType) -> Self {
        Self { exec, trt }
    }
}
