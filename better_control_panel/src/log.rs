use std::{sync::RwLock, thread::Thread};

use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use log::{Level, Log, Record};

///
/// A struct to represent a log message.
///
#[derive(Debug)]
pub struct LogMessage {
    pub thread: Thread,
    pub level: Level,
    pub message: String,
    pub local_time: DateTime<Local>,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

impl LogMessage {
    pub fn new(record: &Record) -> Self {
        Self {
            thread: std::thread::current(),
            level: record.level(),
            message: record.args().to_string(),
            local_time: Local::now(),
            module_path: record.module_path().map(|s| s.to_string()),
            file: record.file().map(|s| s.to_string()),
            line: record.line(),
        }
    }
}

impl std::fmt::Display for LogMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{} {} {} {}]: {}",
            self.local_time,
            self.thread.name().unwrap_or_default(),
            self.level,
            self.module_path.as_deref().unwrap_or_default(),
            self.message
        )
    }
}

type Buffer = Vec<LogMessage>;

lazy_static! {
    static ref LOGGER: RwLock<Buffer> = RwLock::new(vec![]);
}

pub fn write_global_buffer(log_message: LogMessage) {
    let mut buffer = LOGGER.write().unwrap();
    buffer.push(log_message);
}

///
/// Read the global buffer of log messages.
///
pub fn read_global_buffer<'a>() -> std::sync::LockResult<std::sync::RwLockReadGuard<'a, Buffer>> {
    LOGGER.read()
}
