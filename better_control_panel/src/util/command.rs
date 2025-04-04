use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Exec { app_id: String, script: String },
    Args(Vec<String>),
    Exit,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReciverCommand {
    ExecResult { result: String },
    Args(Vec<String>),
    Exit,
}
