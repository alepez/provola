use lazy_static::lazy_static;
use provola_core::{Error, Reporter, TestResult};
use strum_macros::{Display, EnumString, IntoStaticStr};

#[cfg(feature = "terminalreporter")]
pub use provola_terminalreporter::TerminalReporter;

#[cfg(feature = "terminalreporter")]
pub use provola_terminalreporter::ColorfulTerminalReporter;

fn make<T: Reporter + Default + 'static>() -> Result<Box<dyn Reporter>, Error> {
    Ok(Box::new(T::default()))
}

pub fn make_reporter(rt: ReporterType) -> Result<Box<dyn Reporter>, Error> {
    match rt {
        ReporterType::Basic => make::<BasicReporter>(),

        #[cfg(feature = "terminalreporter")]
        ReporterType::Terminal => make::<TerminalReporter>(),

        #[cfg(feature = "terminalreporter")]
        ReporterType::ColorfulTerminal => make::<ColorfulTerminalReporter>(),
    }
}

#[derive(Debug, EnumString, IntoStaticStr, Clone, Copy, Display)]
pub enum ReporterType {
    Basic,
    #[cfg(feature = "terminalreporter")]
    Terminal,
    #[cfg(feature = "terminalreporter")]
    ColorfulTerminal,
}

#[derive(Default)]
pub struct BasicReporter;

impl Reporter for BasicReporter {
    fn report(&self, result: TestResult) -> Result<(), provola_core::ReporterError> {
        match result {
            TestResult::Pass(_) => println!("PASS"),
            TestResult::Fail(_) => println!("FAIL"),
        }
        Ok(())
    }
}

// As default reporter, we use ColorfulTerminal if terminalreporter feature
// is enabled, otherwise we fallback to Basic reporter.

#[cfg(feature = "terminalreporter")]
pub const DEFAULT_REPORTER: ReporterType = ReporterType::ColorfulTerminal;

#[cfg(not(feature = "terminalreporter"))]
pub const DEFAULT_REPORTER: ReporterType = ReporterType::Basic;

lazy_static! {
    pub static ref DEFAULT_REPORTER_STR: &'static str = DEFAULT_REPORTER.into();
}
