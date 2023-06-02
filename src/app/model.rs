use relm4::prelude::*;
use relm4_components::open_button::OpenButton;
use serde_json::Value;

#[derive(Debug)]
pub struct AppModel {
    pub log_file: Option<LogFile>,
    pub open_button: Controller<OpenButton>,
}

#[derive(Debug)]
pub struct LogFile {
    pub path: String,
    pub lines: Vec<LogLine>,
}

#[derive(Debug)]
pub struct LogLine {
    pub message: Value,
}
