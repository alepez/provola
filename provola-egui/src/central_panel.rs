use eframe::egui::*;
use provola_core::{
    test_runners::FullyQualifiedTestCase, AvailableTests, Failure, Reason, Report, TestCase,
    TestResult, TestSuite,
};

pub fn show(
    ui: &mut Ui,
    test_result: &Option<TestResult>,
    available_tests: &Option<AvailableTests>,
) {
    if let Some(test_result) = test_result {
        show_result(ui, test_result, available_tests);
    } else {
        show_no_result(ui, available_tests);
    }
}

fn show_no_result(ui: &mut Ui, available_tests: &Option<AvailableTests>) {
    if let Some(available_tests) = available_tests {
        show_available_tests(ui, available_tests);
    } else {
        ui.label("No results");
    }
}

// TODO Merge test_result/available_tests to show even ignored/disabled tests
fn show_result(ui: &mut Ui, test_result: &TestResult, _available_tests: &Option<AvailableTests>) {
    match test_result {
        TestResult::Pass(result) => show_result_pass(ui, result),
        TestResult::Fail(result) => show_result_fail(ui, result),
    }
}

fn show_result_pass(ui: &mut Ui, reason: &Reason) {
    // TODO summary
    show_reason(ui, reason);
}

fn show_result_fail(ui: &mut Ui, reason: &Reason) {
    // TODO summary
    show_reason(ui, reason);
}

fn show_reason(ui: &mut Ui, reason: &Reason) {
    match reason {
        Reason::Unknown => show_reason_unknown(ui),
        Reason::Generic(msg) => show_reason_generic(ui, msg),
        Reason::NotExpected { actual, expected } => show_reason_not_expected(ui, actual, expected),
        Reason::Report(report) => show_reason_report(ui, report),
    }
}

fn show_reason_unknown(_ui: &mut Ui) {
    // TODO
}

fn show_reason_generic(_ui: &mut Ui, _msg: &str) {
    // TODO
}

fn show_reason_not_expected(_ui: &mut Ui, _actual: &str, _expected: &str) {
    // TODO
}

fn show_reason_report(ui: &mut Ui, report: &Report) {
    if let Some(_name) = &report.name {
        // log::debug!("report: {}", &name);
    }

    for testsuite in &report.testsuites {
        show_testsuite(ui, testsuite);
    }
}

fn symbol_and_name(ok: bool, name: &str) -> String {
    let symbol = if ok { "✔" } else { "✖" };
    format!("{} {}", symbol, name)
}

fn show_testsuite(ui: &mut Ui, testsuite: &TestSuite) {
    let ok = testsuite.failures.unwrap_or(0) == 0;
    let name = symbol_and_name(ok, &testsuite.name);

    CollapsingHeader::new(&name)
        .default_open(true)
        .show(ui, |ui| {
            for testcase in &testsuite.testcases {
                show_testcase(ui, testcase);
            }
        });
}

fn show_testcase(ui: &mut Ui, testcase: &TestCase) {
    let ok = testcase.failures.is_empty();
    let name = symbol_and_name(ok, &testcase.name);

    CollapsingHeader::new(&name)
        .default_open(true)
        .show(ui, |ui| {
            for failure in &testcase.failures {
                show_failure(ui, failure);
            }
        });
}

fn show_failure(ui: &mut Ui, failure: &Failure) {
    let msg = &failure.message;
    ui.label(msg);
}

fn show_available_tests(ui: &mut Ui, available_tests: &AvailableTests) {
    for test_suite in available_tests.test_suites() {
        show_available_test_suite(ui, test_suite);
    }
}

fn show_available_test_suite(
    ui: &mut Ui,
    t: (
        &provola_core::test_runners::TestSuite,
        &std::vec::Vec<FullyQualifiedTestCase>,
    ),
) {
    let (test_suite, test_cases) = t;
    let name = &test_suite.0;
    CollapsingHeader::new(name)
        .default_open(true)
        .show(ui, |ui| {
            for testcase in test_cases.iter() {
                show_available_test_case(ui, testcase);
            }
        });
}

fn show_available_test_case(ui: &mut Ui, testcase: &FullyQualifiedTestCase) {
    let name = &testcase.test_case.0;
    ui.label(name);
    // TODO Checkbox for selecting test
}
