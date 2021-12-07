use provola_core::test_runners::{TestRunner, TestRunnerInfo, TestRunnerType};

pub fn make_test_runner(info: TestRunnerInfo) -> Box<dyn TestRunner> {
    match info.trt {
        TestRunnerType::GoogleTest => {
            todo!()
        }
    }
}
