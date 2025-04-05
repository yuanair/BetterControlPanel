use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Exec { app_id: String, script: String },
    Args(Vec<String>),
    Exit,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReciverCommand {
    Log {
        app_id: String,
        message: crate::log::LogMessageBuf,
    },
    ExecResult {
        result: String,
    },
    Args(Vec<String>),
    Exit,
}
