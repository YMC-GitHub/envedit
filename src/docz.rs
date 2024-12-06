use qsv_docopt::Docopt;
use serde::Deserialize;

const USAGE: &'static str = "
edit value in os env in window

Usage:
  editenv --name <name> --value <value> [--method=<get> | --api=<osenv> -h | --help | -v | --version]
  editenv <api> <method> <name> [value]
  editenv (-h | --help)
  editenv (-v | --version)

Options:
  -h --help     Show this help.
  -v --version     Show version.
  --api=<api>   The api to edit env [default: osenv].
  --method=<method>     The method to edit env [default: get].
  --name=<name>   The name to edit.
  --value=<kv>    The value to set.
";

#[derive(Debug, Deserialize)]
pub struct Args {
    flag_name: String,
    flag_value: String,
    flag_method: String,
    flag_api: String,
    arg_api: Option<String>,
    arg_method: Option<String>,
    arg_name: Option<String>,
    arg_value: Option<String>,
    // flag_speed: isize,
    // flag_drifting: bool,
    // arg_name: Vec<String>,
    // arg_x: Option<i32>,
    // arg_y: Option<i32>,
    // cmd_ship: bool,
    // cmd_mine: bool,
}
pub fn load() -> Args {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    args
}
pub fn info(args: Args) {
    println!("{:?}", args);
    println!("  Names: {:?}", args.arg_name);
}

// use it in project main.rs
// mod docz;
// use docz::{info, load, Args};
// fn main() {
//     let args: Args = load();
//     info(args);
// }
