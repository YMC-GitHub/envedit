use clap::{arg, command, Command};
// use std::path::PathBuf;
use std::string::String;

/// Configuration data for editenv.
pub struct Config {
    /// The method to editenv , set/get.
    pub method: String,

    /// The name field in env store.
    pub name: String,

    /// The value field in env store.
    pub value: String,
    // the value type of value.
    // pub vtype: String,

    // the name space in of name.
    // pub ns: Option<String>,

    // the span char to split name space in of name.
    // pub ns_span: Option<String>,
}

impl Config {
    /// Create a new config using command line arguments.
    pub fn new() -> Config {
        let args = command!()
            .arg(arg!(-n --name <name> "the name to edit env"))
            .arg(arg!(-v --value <value> "the value to bind to name"))
            .arg(arg!(-m --method <method> "the method to edit env, one of them: set|get"))
            .get_matches();

        // _,flag,extras
        let argv: Vec<String> = args
            .get_many::<String>("input")
            .unwrap()
            .map(String::to_string)
            .collect();

        // PathBuf::from(str)
        let name = match args.get_one::<String>("name") {
            Some(val) => String::to_string(val),
            None => argv[1].clone(),
        };

        let value = match args.get_one::<String>("value") {
            Some(val) => String::to_string(val),
            None => "".to_string(),
        };

        let method = match args.get_one::<String>("method") {
            Some(val) => String::to_string(val).to_lowercase(),
            None => "set".to_string(),
        };

        // let parts: Vec<&str> = input_str.split(',').collect();

        Config {
            name: name.to_string(),
            value: value.to_string(),
            method: method,
        }
    }
}

// u_arg,f_arg,e_arg

pub fn demo() {
    let cmd = Command::new("prog").args(&[
        arg!(--config <FILE> "a required file for the configuration and no short"),
        arg!(-d --debug ... "turns on debugging information and allows multiples"),
        arg!([input] "an optional input file to use"),
    ]);

    let m = cmd
        .try_get_matches_from(["prog", "--config", "file.toml"])
        .unwrap();
    assert_eq!(m.get_one::<String>("config").unwrap(), "file.toml");
    assert_eq!(*m.get_one::<u8>("debug").unwrap(), 0);
    assert_eq!(m.get_one::<String>("input"), None);
}
