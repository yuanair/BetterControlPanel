use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Exec(String),
    Args(Vec<String>),
    Exit,
}
