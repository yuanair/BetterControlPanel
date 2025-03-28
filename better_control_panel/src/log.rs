use std::sync::RwLock;

use lazy_static::lazy_static;
use log::{Level, Record};

///
/// A struct to represent a log message.
///
#[derive(Debug)]
pub struct LogMessage {
    pub thread_id: std::thread::ThreadId,
    pub level: Level,
    pub message: String,
    pub timestamp: std::time::SystemTime,
}

impl LogMessage {
    pub fn new(record: &Record) -> Self {
        Self {
            thread_id: std::thread::current().id(),
            level: record.level(),
            message: record.args().to_string(),
            timestamp: std::time::SystemTime::now(),
        }
    }
}

impl std::fmt::Display for LogMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{:?} {:?} {}]: {}",
            self.timestamp, self.thread_id, self.level, self.message
        )
    }
}

type Buffer = Vec<LogMessage>;

lazy_static! {
    static ref LOGGER: RwLock<Buffer> = RwLock::new(vec![]);
}

pub fn write_global_buffer(record: &Record) {
    let mut buffer = LOGGER.write().unwrap();
    buffer.push(LogMessage::new(record));
}

///
/// Read the global buffer of log messages.
///
pub fn read_global_buffer<'a>() -> std::sync::LockResult<std::sync::RwLockReadGuard<'a, Buffer>> {
    LOGGER.read()
}
