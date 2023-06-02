use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AppInput {
    Open(PathBuf),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Outbox {
    FilePicked(String),
}
