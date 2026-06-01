use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize)]
pub enum MessageKind {
    #[serde(rename = "stdout")] Stdout,
    #[serde(rename = "stderr")] Stderr,
}

#[derive(Debug, Serialize)]
pub struct Message {
    pub kind: MessageKind,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ExecResult {
    pub messages: Vec<Message>,
    pub exitcode: i32,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub src: String,
    pub stdin: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    UnknownError,
    #[serde(rename = "time-limit-exceeded")]
    TimeLimitExceeded,
    #[serde(rename = "mem-limit-exceeded")]
    MemLimitExceeded,
}

#[derive(Debug, Serialize)]
pub struct Output {
    pub status:   Status,
    pub compiler: ExecResult,
    pub runtime:  Option<ExecResult>,
}
