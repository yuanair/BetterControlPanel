use std::{
    sync::{RwLock, RwLockWriteGuard},
    thread::Thread,
};

use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use log::{Level, Record, SetLoggerError};
use serde::{Deserialize, Serialize};

///
/// logger
///
struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            LOGGER.write().unwrap().push(LogMessage::new(record));
        }
    }
    fn flush(&self) {}
}

///
/// Initialize the global logger.
///
pub fn init() -> Result<(), SetLoggerError> {
    static LOGGER: Logger = Logger;
    log::set_logger(&LOGGER)
}

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

///
/// A struct to represent a log message in binary format.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMessageBuf {
    pub thread_name: Option<String>,
    pub thread_id: Option<String>,
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
    pub fn to_buf(&self) -> LogMessageBuf {
        LogMessageBuf {
            thread_name: self.thread.name().map(|s| s.to_string()),
            thread_id: Some(format!("{:?}", self.thread.id())),
            level: self.level,
            message: self.message.clone(),
            local_time: self.local_time,
            module_path: self.module_path.clone(),
            file: self.file.clone(),
            line: self.line,
        }
    }
}

impl std::fmt::Display for LogMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[cfg(feature = "thread_id_value")]
        {
            write!(
                f,
                "[{} {} {} {} {}]: {}",
                self.local_time,
                self.thread.name().unwrap_or_default(),
                self.thread.id().as_u64(),
                self.level,
                self.module_path.as_deref().unwrap_or_default(),
                self.message
            )
        }
        #[cfg(not(feature = "thread_id_value"))]
        {
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
}

impl std::fmt::Display for LogMessageBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{} {} {} {} {}]: {}",
            self.local_time,
            self.thread_name.as_deref().unwrap_or_default(),
            self.thread_id.as_deref().unwrap_or_default(),
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

///
/// Clear the global buffer of log messages.
///
pub fn clear_global_buffer() -> Result<(), std::sync::PoisonError<RwLockWriteGuard<'static, Buffer>>>
{
    Ok(LOGGER.write()?.clear())
}

///
/// Try to clear the global buffer of log messages.
///
pub fn try_clear_global_buffer()
-> Result<(), std::sync::TryLockError<RwLockWriteGuard<'static, Buffer>>> {
    Ok(LOGGER.try_write()?.clear())
}

///
/// Read the global buffer of log messages.
///
pub fn read_global_buffer<'a>() -> std::sync::LockResult<std::sync::RwLockReadGuard<'a, Buffer>> {
    LOGGER.read()
}

///
/// Try to read the global buffer of log messages.
///
pub fn try_read_global_buffer()
-> std::sync::TryLockResult<std::sync::RwLockReadGuard<'static, Buffer>> {
    LOGGER.try_read()
}

pub fn pop_global_buffer()
-> Result<Option<LogMessage>, std::sync::PoisonError<RwLockWriteGuard<'static, Buffer>>> {
    Ok(LOGGER.write()?.pop())
}

pub fn try_pop_global_buffer()
-> Result<Option<LogMessage>, std::sync::TryLockError<RwLockWriteGuard<'static, Buffer>>> {
    Ok(LOGGER.try_write()?.pop())
}

pub fn redirect_panic_to_log() {
    std::panic::set_hook(Box::new(move |info| {
        log::error!(target: "panic", "{}", info);
    }));
}
