extern crate clap;

use std::ffi::OsString;

use clap::{App, Arg, };

#[derive(Debug, PartialEq)]
pub struct J1DumpArgs {
    pub bin_file_name: String,
    pub script_file_name: String,
    pub addr_start: u16,
    pub addr_end: u16,
}

#[allow(dead_code)]
impl J1DumpArgs {
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
            .about("J1 cpu memory dump")
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

        // define memory start_address command line option
        let addr_start_option = Arg::with_name("addr_start")
            .long("start") // allow --start
            .takes_value(true)
            .default_value("0x0000")
            .help("Start memory address to dump, example 0X0000")
            .required(false);


        // define memory end_address command line option
        let addr_end_option = Arg::with_name("addr_end")
            .long("end") // allow --start
            .takes_value(true)
            .default_value("0x2000")
            .help("End memory address to dump, example 0X2000")
            .required(false);


        let app = app
            .arg(bin_file_name_option)
            .arg(script_file_name_option)
            .arg(addr_start_option)
            .arg(addr_end_option);
        let matches = app.get_matches_from_safe(args)?;
        let bin_file_name = matches.value_of("bin_file").unwrap_or("").to_string();
        let script_file_name = matches.value_of("script_file").unwrap_or("").to_string();
        let addr_start = clap::value_t!(matches.value_of("addr_start"), u16).unwrap_or_else(|e| e.exit());
        let addr_end = clap::value_t!(matches.value_of("addr_end"), u16).unwrap_or_else(|e| e.exit());

        if addr_start > addr_end ||  addr_end > 0x2000 {
            return Err(clap::Error::with_description("Invalid addresses", clap::ErrorKind::InvalidValue))
        }
        Ok(J1DumpArgs { bin_file_name, script_file_name, addr_start, addr_end })
    }
}
