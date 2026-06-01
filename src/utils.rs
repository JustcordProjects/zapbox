use std::io::{self, BufRead, BufReader, Read};
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Sender};
use std::thread;

use crate::models::{Message, Status};
use crate::models::{ExecResult, MessageKind};

pub fn exitcode2status(code: i32) -> Status {
    match code {
        0         => Status::Success,
        137 | 139 => Status::MemLimitExceeded,
        124 | 152 => Status::TimeLimitExceeded,
        _         => Status::UnknownError,
    }
}

fn spawn_reader<R: Read + Send + 'static>(stream: R, kind: MessageKind, sender: Sender<Message>) {
    thread::spawn(move || {
        for line in BufReader::new(stream).lines() {
            if let Ok(content) = line {
                let _ = sender.send(Message { kind, content });
            }
        }
    });
}

pub fn run_and_capture(cmd: &mut Command) -> io::Result<ExecResult> {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let (sender, receiver) = mpsc::channel();

    spawn_reader(stdout, MessageKind::Stdout, sender.clone());
    spawn_reader(stderr, MessageKind::Stderr, sender.clone());
    
    drop(sender);

    let mut messages = Vec::new();
    for msg in receiver {
        messages.push(msg);
    }

    let status = child.wait()?;
    Ok(ExecResult {
        messages: messages,
        exitcode: status.code().unwrap_or(128 + status.signal().unwrap_or(1))
    })
}
