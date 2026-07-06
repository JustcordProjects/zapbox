mod models;
use models::*;

mod utils;
use utils::*;

use std::env;
use std::fs::File;
use std::io::Write;
use std::process::{
    self as proc,
    Command,
};

use spvec::spvec;

const ROOTFS_DIR: &str = "rootfs";
const ROOTFS_URL: &str = "https://github.com/termux/proot-distro/releases/download/v4.11.0/ubuntu-noble-x86_64-pd-v4.11.0.tar.xz";

const TIMEOUT_SEC: i32 = 5;
const LIMIT_FLAGS: &[&str] = &[
    "--as=1000000000",
    "--cpu=10",
    "--nproc=2048",
    "--fsize=100000000",
];

fn usage() {
    // did you know that println![] or println!{} is valid in rust?
    // most useless facts #0
    eprintln!{r#"
usage: zapbox <command> [args...]
commands:
    zapbox run <json-data...>    - compile and run given source code (outputs json to stdout)
    zapbox setup                 - download and prepare the rootfs using proot
"#};
}

fn setup() -> anyhow::Result<()> {
    if std::fs::metadata(format!("{ROOTFS_DIR}/bin/sh")).is_err() {
        let _ = std::fs::remove_dir_all(ROOTFS_DIR);
        let status = Command::new("sh")
            .arg("-c")
            .arg(format!("mkdir -p {ROOTFS_DIR} && curl -sSLf {ROOTFS_URL} | tar -xJC {ROOTFS_DIR} --strip-components=1 --exclude='dev' --exclude='proc' --exclude='sys'"))
            .status()?;

        if !status.success() {
            let _ = std::fs::remove_dir_all(ROOTFS_DIR);
            eprintln!("failed to download or extract rootfs");
            proc::exit(1);
        }
    }

    let _ = Command::new("chmod").args(["-R", "u+w", ROOTFS_DIR]).status();
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("grep -q '^staff:' {ROOTFS_DIR}/etc/group || echo 'staff:x:50:' >> {ROOTFS_DIR}/etc/group"))
        .status();

    let status = Command::new("unshare")
        .args(["-n", "proot"])
        .args(["-R", ROOTFS_DIR, "-0", "-b", "/dev", "-b", "/proc", "-b", "/sys"])
        .args(["/usr/bin/sh", "-c", include_str!("setup.sh")])
        .status()?;

    if !status.success() {
        eprintln!("failed to setup rootfs dependencies");
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

    let mount_arg   = format!("{}:/workspace", dir.display());
    let timeout_arg = format!("{TIMEOUT_SEC}s");

    let proot_base: Vec<&str> = vec![
        "proot", "-R", ROOTFS_DIR, "-0", 
        "-b", "/dev", "-b", "/proc", "-b", "/sys", 
        "-b", &mount_arg, "-w", "/workspace",
    ];

    // now we have fancy spread syntax, amazing i know
    let compile_args: Vec<&str> = spvec![
        "prlimit", ...LIMIT_FLAGS, ...&proot_base,
        "/opt/zap/zapc", "src.zp", "-o", "exe",
    ];

    let compile_result = run_and_capture(
        Command::new("timeout")
            .arg(&timeout_arg)
            .args(compile_args)
    )?;

    if compile_result.exitcode != 0 {
        return Ok(Output {
            status: map_exitcode(compile_result.exitcode),
            compiler: compile_result,
            runtime: None,
        });
    }

    let runtime_args: Vec<&str> = spvec![
        "prlimit", ...LIMIT_FLAGS, ...&proot_base,
        ... if input.stdin.is_some() {
                vec!["/usr/local/bin/pipe", "stdin.txt", "./exe"]
            } else {
                vec!["./exe"]
            },
    ];

    let mut runtime_cmd = Command::new("timeout");
    runtime_cmd.arg(&timeout_arg).args(runtime_args);

    let runtime_result = run_and_capture(&mut runtime_cmd)?;

    Ok(Output {
        status:   map_exitcode_ignore_unknown(runtime_result.exitcode),
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
        "setup" => setup(),
        "run"   => run(&args[2]),
        _ => {
            eprintln!("unknown command: {}", args[1]);
            usage();
            Ok(())
        }
    }
}
