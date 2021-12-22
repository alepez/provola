use super::Opt;
use provola_core::*;
use std::convert::TryFrom;
use std::path::Path;
use std::time::Duration;

fn run_forever(opt: &Opt, watch_file: &Path) -> Result<(), Error> {
    run_once_or_log_error(opt);

    let watch_opt = WatchOptions {
        file: watch_file.to_path_buf(),
        debounce_time: Duration::from_secs(1),
    };

    Watcher::try_from(watch_opt)?.watch(&mut || {
        run_once_or_log_error(opt);
    })?;

    Ok(())
}

fn run_once_or_log_error(opt: &Opt) {
    if let Err(e) = run_once(opt) {
        log::error!("{}", e);
    }
}

fn run_once(opt: &Opt) -> Result<(), Error> {
    let action = Action::try_from(opt)?;
    let result = action.run()?;
    let reporter = opt.reporter()?;

    reporter.report(result)?;

    Ok(())
}

fn list_tests(opt: &Opt) -> Result<(), Error> {
    let action = Action::try_from(opt)?;

    let list = match action {
        Action::TestRunner(tr) => tr.list()?,
        _ => todo!(),
    };

    for test in list.iter() {
        println!("{} {}", test.id, test);
    }

    Ok(())
}

pub(crate) fn run(opt: &Opt) -> Result<(), Error> {
    if opt.list {
        list_tests(opt)
    } else if let Some(watch_files) = &opt.watch {
        run_forever(opt, watch_files)
    } else {
        run_once(opt)
    }
}
