use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub enum MessageKind {
    Stdout,
    Stderr,
}

#[derive(Debug, Serialize)]
pub struct Message {
    kind: MessageKind,
    content: String,
}

#[derive(Debug, Serialize)]
pub struct ExecResult {
    messages: Vec<Message>,
    exitcode: i32,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    src: String,
    stdin: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum Status {
    Success,
    TimeLimitExceeded,
    MemLimitExceeded,
}

#[derive(Debug, Serialize)]
pub struct Output {
    status:   Status,
    compiler: ExecResult,
    runtime:  Option<ExecResult>,
}
