mod models;
use models::*;

mod utils;
use utils::*;

use std::env;
use std::fs::File;

use std::process::{
    self as proc,
    Command,
};

use std::io::Write;

const IMAGE_NAME: &str = "zapbox-image";

const TIMEOUT_SEC: i32 = 5;
const LIMIT_FLAGS: &[&str] = &[
    "--net", "none",
    "--memory", "256m",
];

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

fn do_run(input: Input) -> anyhow::Result<Output> {
    let dir = tempfile::tempdir()?;
    let dir = dir.path();

    let mut src_file = File::create(dir.join("src.zp"))?;
    write!(src_file, "{}", &input.src)?;

    if let Some(stdin) = &input.stdin { 
        let mut stdin_file = File::create(dir.join("stdin.txt"))?;
        write!(stdin_file, "{}", stdin)?;
    }

    let mount_arg   = format!("{}:/workspace:Z", dir.display());
    let timeout_arg = format!("{TIMEOUT_SEC}s");

    // i wish rust had a spread operator
    let mut compile_args: Vec<&str> = vec![];
    compile_args.extend(["podman", "run", "--rm"]);
    compile_args.extend(LIMIT_FLAGS);
    compile_args.extend(["-v", &mount_arg]);
    compile_args.extend([IMAGE_NAME]);
    compile_args.extend(["zapc", "src.zp", "-o", "exe"]);

    let compile_result = run_and_capture(
        Command::new("timeout")
            .arg(&timeout_arg)
            .args(compile_args)
    )?;

    if compile_result.exitcode != 0 {
        return Ok(Output {
            status: exitcode2status(compile_result.exitcode),
            compiler: compile_result,
            runtime: None,
        });
    }

    let mut runtime_args: Vec<&str> = vec![];
    runtime_args.extend(["podman", "run", "--rm"]);
    runtime_args.extend(LIMIT_FLAGS);
    runtime_args.extend(["-v", &mount_arg]);
    runtime_args.extend([IMAGE_NAME]);

    if input.stdin.is_some() {
        runtime_args.extend(["pipe", "stdin.txt", "./exe"]);
    } else {
        runtime_args.push("./exe");
    }

    let mut runtime_cmd = Command::new("timeout");
    runtime_cmd.arg(&timeout_arg).args(runtime_args);

    let runtime_result = run_and_capture(&mut runtime_cmd)?;

    Ok(Output {
        status:   exitcode2status(runtime_result.exitcode),
        compiler: compile_result,
        runtime:  Some(runtime_result),
    })
}

fn run(arg: &str) -> anyhow::Result<()> {
    let input: Input = serde_json::from_str(arg)?;
    let output: Output = do_run(input)?;
    let json = serde_json::to_string(&output)?;
    println!("{json}");
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
