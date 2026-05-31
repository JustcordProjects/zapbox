#![allow(dead_code)]

#[derive(Debug)]
enum MessageKind {
    Stdout,
    Stderr,
}

#[derive(Debug)]
struct Message {
    kind: MessageKind,
    content: String,
}

#[derive(Debug)]
struct ExecResult {
    messages: Vec<Message>,
    exitcode: i32,
}

#[derive(Debug)]
struct Input {
    src: String,
    stdin: String,
}

#[derive(Debug)]
enum Status {
    Success,
    TimeLimitExceeded,
    MemLimitExceeded,
}

#[derive(Debug)]
struct Output {
    status:   Status,
    compiler: ExecResult,
    runtime:  Option<ExecResult>,
}
