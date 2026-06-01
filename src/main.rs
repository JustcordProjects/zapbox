mod models;
use models::*;

use std::env;
use std::fs::File;
use std::process as proc;

use std::io::Write;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: zapc-wrapper <json-data...>");
        proc::exit(1);
    }

    let input: Input = serde_json::from_str(&args[1])?;
    dbg!(&input);

    let dir = tempfile::tempdir()?;
    let dir = dir.path();

    let mut src_file = File::create(dir.join("src.zp"))?;
    write!(src_file, "{}", &input.src)?;

    if let Some(stdin) = &input.stdin { 
        let mut stdin_file = File::create(dir.join("stdin.txt"))?;
        write!(stdin_file, "{}", stdin)?;
    }

    Ok(())
}
