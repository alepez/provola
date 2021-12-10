use provola_core::test_runners::{TestRunner, TestRunnerInfo, TestRunnerType};

pub fn make_test_runner(info: TestRunnerInfo) -> Box<dyn TestRunner> {
    let test_runner_type = info.trt;
    match test_runner_type {
        #[cfg(feature = "googletest")]
        TestRunnerType::GoogleTest => provola_googletest::TestRunner::from_executable(info.exec),
        _ => todo!()
    }
}
