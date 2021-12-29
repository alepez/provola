use eframe::egui::*;
use provola_core::{CoreFailure, CoreReport, CoreTestCase, CoreTestSuite, Reason, TestResult};

pub fn show(ui: &mut Ui, test_result: Option<&TestResult>) {
    if let Some(test_result) = test_result {
        show_result(ui, test_result);
    } else {
        show_no_result(ui);
    }
}

fn show_no_result(ui: &mut Ui) {
    ui.label("No results");
}

// TODO Merge test_result/available_tests to show even ignored/disabled tests
fn show_result(ui: &mut Ui, test_result: &TestResult) {
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

fn show_reason_report(ui: &mut Ui, report: &CoreReport) {
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

fn show_testsuite(ui: &mut Ui, testsuite: &CoreTestSuite) {
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

fn show_testcase(ui: &mut Ui, testcase: &CoreTestCase) {
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

fn show_failure(ui: &mut Ui, failure: &CoreFailure) {
    let msg = &failure.message;
    ui.label(msg);
}
