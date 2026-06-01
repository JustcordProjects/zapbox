use std::io::{self, BufRead, BufReader, Read};
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Sender};
use std::thread;

use crate::models::Message;
use crate::models::{ExecResult, MessageKind};


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

    child.wait()?;
    Ok(ExecResult {
        messages: messages,
        // from what i understand if this is None
        // then process was terminated by a signal
        exitcode: child.wait()?.code().unwrap_or(1)
    })
}
