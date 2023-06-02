use std::path::PathBuf;

use futures::prelude::*;
use relm4::gtk::glib::DateTime;
use serde_json::Value;
use thiserror::Error;
use tokio::io::{self, AsyncBufReadExt};
use tokio::{fs::File, io::BufReader};
use tokio_stream::wrappers::LinesStream;
#[derive(Error, Debug)]
pub enum LogParseError {
    #[error("Could not open file")]
    FileOpen(io::Error),
}

#[derive(Debug)]
pub struct LogLine {
    pub line_number: usize,
    pub value: LineValue,
    pub timestamp: Option<DateTime>,
}

#[derive(Debug)]
pub enum LineValue {
    Json(Value),
    Text(String),
}

pub async fn parse_logfile(path: PathBuf) -> Result<Vec<LogLine>, LogParseError> {
    let f = File::open(path).await.map_err(LogParseError::FileOpen)?;
    let lines = BufReader::new(f).lines();
    let lines_stream = LinesStream::new(lines);
    let ret = lines_stream
        .enumerate()
        .filter_map(|line| async move {
            if let (line_number, Ok(line)) = line {
                let json_value = serde_json::from_str(&line);
                if let Ok(json_value) = json_value {
                    Some(LogLine {
                        line_number,
                        value: LineValue::Json(json_value),
                        timestamp: None,
                    })
                } else {
                    Some(LogLine {
                        line_number,
                        value: LineValue::Text(line),
                        timestamp: None,
                    })
                }
            } else {
                None
            }
        })
        .collect()
        .await;

    Ok(ret)
}
