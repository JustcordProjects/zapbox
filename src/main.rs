mod models;
use models::*;

use std::env;
use std::fs::File;

use std::process::{
    self as proc,
    Command,
};

use std::io::Write;

const IMAGE_NAME: &str = "zapbox-image";

fn usage() {
    // did you know that println![] or println!{} is valid in rust?
    // most useless facts #0
    eprintln!{r#"
usage: zapbox <command> [args...]
commands:
    zapbox run <json-data...>    - compile and run given source code (outputs json to stdout)
    zapbox build                 - build the podman image (with name {IMAGE_NAME})
"#};
}

fn build_image() -> anyhow::Result<()> {
    let status = Command::new("podman")
        .args(["build", "-t", IMAGE_NAME, "."])
        .status()?;

    if !status.success() {
        eprintln!("failed to build the container image");
        proc::exit(1);
    }

    Ok(())
}

fn run(arg: &str) -> anyhow::Result<()> {
    let input: Input = serde_json::from_str(arg)?;
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

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("unexpected amount of arguments");
        usage();
        proc::exit(1);
    }

    match args[1].as_str() {
        "build" => build_image(),
        "run"   => run(&args[2]),
        _ => {
            eprintln!("unknown command: {}", args[1]);
            usage();
            Ok(())
        }
    }
}
