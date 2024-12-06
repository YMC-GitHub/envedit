// 1. define mod in main.rs or lib.rs
// mod config;
// mod error;
// mod docz;

// 2. use std lib or this crate's mod
// use config::Config;
// use error::Error;
// use std::{fs, path::PathBuf, process};
use std::path::Path;

// use docz::{info, load, Args};
// fn main() {
//     let args: Args = load();
//     info(args);
// }

use std::env;
use std::process::Command;
use std::str;
use winreg::enums::*;
use winreg::RegKey;

fn set_env_variable(key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env_key = hkcu.open_subkey_with_flags(r"Environment", KEY_ALL_ACCESS)?;

    match value {
        "" => {
            // del when empty value
            env_key.delete_value(key)?;
        }
        _ => {
            // set as other value
            env_key.set_value(key, &value)?;
        }
    }

    // Command-line implementation of permanently setting environment variables
    // Command::new("setx").arg(key).arg(value).status()?;
    let _output = Command::new("setx")
        .arg(key)
        .arg(value)
        .output()
        .expect("Failed to execute setx command");

    // if _output.status.success() {
    //     println!("Environment variable {} set successfully.", key);
    // } else {
    //     println!(
    //         "Failed to set environment variable {}. Error: {}",
    //         key,
    //         String::from_utf8_lossy(&_output.stderr)
    //     );
    // }

    Ok(())
}

fn get_env_variable(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    // let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    // let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion")?;

    // // info program file dir and deive path ? do
    // let pf: String = cur_ver.get_value("ProgramFilesDir")?;
    // let dp: String = cur_ver.get_value("DevicePath")?;
    // println!("ProgramFiles = {}\nDevicePath = {}", pf, dp);

    // HKEY_CURRENT_USER\Environment
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env_path = Path::new("Environment");
    let (regkey, _disp) = hkcu.create_subkey(&env_path)?;
    // match _disp {
    //     REG_CREATED_NEW_KEY => println!("A new key has been created"),
    //     REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
    // }

    // key.set_value("TestSZ", &"written by Rust")?;
    let value: String = regkey.get_value(key)?;
    Ok(value)
}

// fn info_kv(k: &String, v: String) {
//     println!("Value of environment variable '{}': {}", k, v)
// }
fn info_v(_k: &String, v: String) {
    println!("{}", v)
}

#[derive(Debug)]
struct CliAgrs {
    method: String,
    name: String,
    value: String,
}

fn parese_cli_arg_with_std() -> Option<CliAgrs> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <set|get> <variable_name> [value]", args[0]);
        return None;
    }
    // read,parse
    let operation = &args[1];
    let variable_name = &args[2];
    let value = if args.len() < 4 { "" } else { &args[3] };

    Some(CliAgrs {
        method: tos(operation),
        name: tos(variable_name),
        value: tos(value),
    })
}

/// Accepts &str,&String, and returns a String value.
///
/// based on using the syntax sugar of Into trait
fn tos(input: impl Into<String>) -> String {
    input.into()
}

fn call_eenv(args: CliAgrs) {
    let cmd = args.method;
    let kn = &args.name;
    let kv = &args.value;
    match cmd.as_str() {
        "set" => {
            if let Err(e) = set_env_variable(kn, kv) {
                eprintln!("Error setting environment variable: {}", e);
            } else {
                println!("Environment variable '{}' set successfully.", kn);
            }
        }
        "get" => {
            if let Err(e) = get_env_variable(kn) {
                eprintln!("Error getting environment variable: {}", e);
            } else {
                match get_env_variable(kn) {
                    Ok(value) => info_v(kn, value),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        _ => println!("Invalid operation. Use 'set' or 'get'."),
    }
}

fn main() {
    if let Some(args) = parese_cli_arg_with_std() {
        call_eenv(args);
    }
}

// cargo run -- editenv --help
// cargo run -- --help
// cargo run -- -h
// cargo run -- --version
// cargo run -- --name name --value "Ye MianCheng"
// cargo run -- osenv set name "Ye MianCheng"
// cargo run -- osenv set name "Ye MianCheng"
// cargo run -- get cuda_path
// cargo run -- set cuda_path "F:\scoop\apps\cuda\current"

// cargo run -- get OLLAMA_MODELS
// cargo run -- set OLLAMA_MODELS "G:\data\LLM"
// cargo run -- set OLLAMA_MODELS
// cargo run -- set OLLAMA_MODELS "G:\data\ollama\LLM"
