use provola_core::test_runners::TestRunner;
use provola_core::{Error, Executable};
use strum_macros::{Display, EnumString};

fn from_exec<T>(info: TestRunnerInfo) -> Result<Box<dyn TestRunner>, Error>
where
    T: From<Executable> + provola_core::test_runners::TestRunner + 'static,
{
    Ok(Box::new(T::from(info.exec)))
}

pub fn make_test_runner(info: TestRunnerInfo) -> Result<Box<dyn TestRunner>, Error> {
    let test_runner_type = info.trt;
    match test_runner_type {
        #[cfg(feature = "googletest")]
        TestRunnerType::GoogleTest => from_exec::<provola_googletest::TestRunner>(info),
        #[cfg(feature = "catch2")]
        TestRunnerType::Catch2 => from_exec::<provola_catch2::TestRunner>(info),
    }
}

#[derive(
    Debug, EnumString, Clone, Copy, Display, serde::Deserialize, serde::Serialize, PartialEq, Eq,
)]
pub enum TestRunnerType {
    #[cfg(feature = "googletest")]
    GoogleTest,
    #[cfg(feature = "catch2")]
    Catch2,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub struct TestRunnerInfo {
    pub exec: Executable,
    pub trt: TestRunnerType,
}

impl TestRunnerInfo {
    pub fn new(exec: Executable, trt: TestRunnerType) -> Self {
        Self { exec, trt }
    }
}
