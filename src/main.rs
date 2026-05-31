mod models;
use models::*;

use std::env;
use std::process as proc;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: zapc-wrapper <json-data...>");
        proc::exit(1);
    }

    // TODO: better error handling
    let input: Input = serde_json::from_str(&args[1]).expect("invalid input json");
    dbg!(input);
}
