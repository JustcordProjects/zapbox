use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize)]
pub enum MessageKind {
    Stdout,
    Stderr,
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
    Success,
    UnknownError,
    TimeLimitExceeded,
    MemLimitExceeded,
}

#[derive(Debug, Serialize)]
pub struct Output {
    pub status:   Status,
    pub compiler: ExecResult,
    pub runtime:  Option<ExecResult>,
}
