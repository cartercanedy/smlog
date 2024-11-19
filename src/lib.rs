#![doc = include_str!("../README.md")]

use std::sync::Mutex;

use log::{LevelFilter, Level};

pub use log::{self, trace, debug, info, warn, error};

/// A quick-and-dirty logging implementation
pub struct Log;

static LOGGER: Log = Log;
static LOG_LEVEL_FILTER: Mutex<LevelFilter> = Mutex::new(LevelFilter::Off);
static LOG_IGNORE: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// Ignores the module specified by `target`
///
/// # Arguments
///
/// * `target` the module to add to module filter
///    Includes all submodules of `target`
///
/// ```rust
/// use smlog::{Log, LevelFilter, ignore};
///
/// fn main() {
///     // this will disallow logging from any modules within `foo` namespace, e.g. `foo`,
///     // `foo::bar`, `foo::baz`, etc.
///     ignore("foo");
///     Log::init(LevelFilter::Warn);
///
///     // ...
/// }
/// ```
pub fn ignore(target: impl AsRef<str>) {
    LOG_IGNORE.lock().unwrap().push(target.as_ref().into());
}

/// Explicitly allows the module specified by `target`
///
/// # Arguments
///
/// * `target` the module remove from module filter
///
/// # Example
///
/// ```rust
/// use smlog::{Log, LevelFilter, ignore, allow};
/// 
/// fn main() {
///     ignore("foo");
///
///     Log::init(LevelFilter::Warn);
///
///     // ...
///
///     allow("foo");
/// }
/// ```
pub fn allow(target: impl AsRef<str>) {
    let mut ign = LOG_IGNORE.lock().unwrap();

    if let Ok(i) = ign.binary_search(&target.as_ref().into()) {
        ign.remove(i);
    }
}

impl Log {
    /// Initialize logging with the specified severity filter
    ///
    /// # Arguments
    ///
    /// * `level_filter` the max severity for emitted logs
    pub fn init(level_filter: LevelFilter) {
        *LOG_LEVEL_FILTER.lock().unwrap() = level_filter;

        log::set_logger(&LOGGER)
            .map(|_| log::set_max_level(level_filter))
            .unwrap();
    }
}

impl log::Log for Log {
    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let pfx = match record.level() {
            Level::Error => "error",
            Level::Warn => "warning",
            Level::Info => "info",
            Level::Debug => "debug",
            Level::Trace => "trace",
        };

        format!("{}", record.args())
            .lines()
            .for_each(|l| println!("{pfx}: {l}"));
    }

    fn flush(&self) { }

    fn enabled(&self, metadata: &log::Metadata) -> bool {
        *LOG_LEVEL_FILTER.lock().unwrap() >= metadata.level()
            && !LOG_IGNORE.lock().unwrap().iter().fold(false, |disable, ign| disable || metadata.target().starts_with(ign))
    }
}

