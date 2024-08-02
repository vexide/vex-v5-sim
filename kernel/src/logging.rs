use alloc::boxed::Box;
use core::ops::Deref;

use log::{debug, set_logger, set_max_level, LevelFilter, Log, Metadata, SetLoggerError};
use vexide_core::io::Write;

use super::uart;
use crate::drivers::uart::uart1;

pub struct KernelLogger {
    level: LevelFilter,
}

impl KernelLogger {
    pub fn init(log_level: LevelFilter) -> Result<(), SetLoggerError> {
        set_max_level(log_level);
        set_logger(Box::leak(Self::new(log_level)))?;
        debug!("Logging initialized");
        Ok(())
    }

    pub fn new(log_level: LevelFilter) -> Box<Self> {
        Box::new(Self { level: log_level })
    }
}

impl Log for KernelLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            write!(uart1(), "[{}] {}\n", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {}
}
