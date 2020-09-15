extern crate clap;

use std::ffi::OsString;

use clap::{App, Arg};

#[derive(Debug, PartialEq)]
pub struct J1Args {
    pub bin_file_name: String,
    pub script_file_name: String,
    pub repl: bool,
}

#[allow(dead_code)]
impl J1Args {
    pub fn new() -> Self {
        Self::new_from(std::env::args_os().into_iter()).unwrap_or_else(|e| e.exit())
    }

    fn new_from<I, T>(args: I) -> Result<Self, clap::Error>
        where
            I: Iterator<Item=T>,
            T: Into<OsString> + Clone,
    {
        // basic app information
        let app = App::new("j1")
            .version("1.0")
            .about("J1 cpu emulator")
            .author("Roy Crippen");

        // define the binary file name command line option
        let bin_file_name_option = Arg::with_name("bin_file")
            .long("bin") // allow --bin
            .short("b") // allow -b
            .takes_value(true)
            .help("Binary J1 forth imamge to load")
            .required(false);

        // define the forth script file name command line option
        let script_file_name_option = Arg::with_name("script_file")
            .long("script") // allow --script
            .short("s") // allow -s
            .takes_value(true)
            .help("Forth script file to load and execute")
            .required(false);

        // define the repl option, versus logging
        let repl_flag = Arg::with_name("repl")
            .long("repl") // allow --repl
            .short("r") // allow -r
            .help("Run the J1 forth repl")
            .required(false);

        let app = app.arg(bin_file_name_option).arg(script_file_name_option).arg(repl_flag);
        let matches = app.get_matches_from_safe(args)?;
        let bin_file_name = matches.value_of("bin_file").unwrap_or("").to_string();
        let script_file_name = matches.value_of("script_file").unwrap_or("").to_string();
        let mut repl = false;
        if matches.occurrences_of("repl") > 0 {
            repl = true
        }
        Ok(J1Args { bin_file_name, script_file_name, repl })
    }
}
