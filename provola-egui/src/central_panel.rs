use eframe::egui::Ui;
use provola_core::{Reason, Report, TestResult};

pub fn show(ui: &mut Ui, test_result: &Option<TestResult>) {
    if let Some(test_result) = test_result {
        show_result(ui, test_result);
    } else {
        show_no_result(ui);
    }
}

fn show_no_result(ui: &mut Ui) {
    // TODO
}

fn show_result(ui: &mut Ui, test_result: &TestResult) {
    match test_result {
        TestResult::Pass(result) => show_result_pass(ui, result),
        TestResult::Fail(result) => show_result_fail(ui, result),
    }
}

fn show_result_pass(ui: &mut Ui, reason: &Reason) {
    show_reason(ui, reason);
}

fn show_result_fail(ui: &mut Ui, reason: &Reason) {
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

fn show_reason_unknown(ui: &mut Ui) {}

fn show_reason_generic(ui: &mut Ui, msg: &str) {}

fn show_reason_not_expected(ui: &mut Ui, actual: &str, expected: &str) {}

fn show_reason_report(ui: &mut Ui, report: &Report) {
    if let Some(name) = &report.name {
        // writeln!(f, "{}", name.bold())?;
    }

    for testsuite in &report.testsuites {
        // writeln!(f, "  {}", testsuite.name.bold())?;

        for testcase in &testsuite.testcases {
            let ok = testcase.failures.is_empty();
            let symbol = if ok { "✔" } else { "✖" };
            // writeln!(f, "    {} {}", symbol, testcase.name)?;
        }
    }
}
