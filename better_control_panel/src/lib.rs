#![cfg_attr(feature = "thread_id_value", feature(thread_id_value))]
#[cfg(feature = "eframe")]
pub mod eframe;

///
/// Inter-Process Communication
///
pub mod ipc;

///
/// About log
///
pub mod log;

///
/// Some utilities
///
pub mod util;
