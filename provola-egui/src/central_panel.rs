use eframe::egui::*;
use provola_core::{
    report::CoreStatus, CoreFailure, CoreReport, CoreTestCase, CoreTestSuite, Reason, TestResult,
};

pub struct TestsExplorer {}

impl TestsExplorer {
    pub(crate) fn new() -> Self {
        todo!()
    }
}

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

fn symbol(status: CoreStatus) -> &'static str {
    match status {
        CoreStatus::Pass => "✔",
        CoreStatus::Fail => "✖",
        _ => "?",
    }
}

fn from_status_to_color(status: CoreStatus) -> Color32 {
    match status {
        CoreStatus::Pass => Color32::GREEN,
        CoreStatus::Fail => Color32::RED,
        _ => Color32::LIGHT_GRAY,
    }
}

fn symbol_and_name(ok: CoreStatus, name: &str) -> RichText {
    let text = format!("{} {}", symbol(ok), name);
    let color = from_status_to_color(ok);
    RichText::new(text).color(color)
}

fn show_testsuite(ui: &mut Ui, testsuite: &CoreTestSuite) {
    let ok = testsuite.failures.map(|x| x == 0);
    let name = symbol_and_name(ok.into(), &testsuite.name);

    CollapsingHeader::new(name)
        .default_open(true)
        .show(ui, |ui| {
            for testcase in &testsuite.testcases {
                show_testcase(ui, testcase);
            }
        });
}

fn show_testcase(ui: &mut Ui, testcase: &CoreTestCase) {
    let has_failures = !testcase.failures.is_empty();

    if has_failures {
        show_testcase_with_failures(ui, testcase);
    } else {
        show_testcase_without_failures(ui, testcase);
    }
}

fn show_testcase_with_failures(ui: &mut Ui, testcase: &CoreTestCase) {
    let status = testcase.status;
    let name = symbol_and_name(status, &testcase.name);

    CollapsingHeader::new(name)
        .default_open(true)
        .show(ui, |ui| {
            for failure in &testcase.failures {
                show_failure(ui, failure);
            }
        });
}

fn show_testcase_without_failures(ui: &mut Ui, testcase: &CoreTestCase) {
    let status = testcase.status;
    let name = symbol_and_name(status, &testcase.name);

    let res = CollapsingHeader::new(name)
        .default_open(false)
        .show(ui, |_ui| {});

    res.header_response.context_menu(|ui| {
        if ui.button("Enable only this").clicked() {
            log::info!("Enable only test {}", testcase.fqtc.unwrap());
            ui.close_menu();
        }
    });
}

fn show_failure(ui: &mut Ui, failure: &CoreFailure) {
    let msg = &failure.message;
    ui.label(msg);
}
