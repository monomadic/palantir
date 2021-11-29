#[macro_use]
extern crate log;
use log::{LogLevel, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Warn
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            // I can probably change colors here
            println!("{} - {}", record.level(), record.args());
        }
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Warn);
        Box::new(SimpleLogger)
    })
}
