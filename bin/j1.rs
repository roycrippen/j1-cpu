use std::io::{Error, ErrorKind};
extern crate clap;
use std::ffi::OsString;
use clap::{App, Arg};
use j1::cpu::CPU;
use j1::j1e_bin;
use j1::utils::{read_binary, read_forth_source, write_log_file};
use std::io;

#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    let args = Args::new();
    println!("Starting j1...\n");

    // println!("{:?}", j1_args);
    if !args.repl && args.script_file_name.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "Must provide a script file if not running repl"))
    }

    // read a forth script file
    let mut script: Vec<u8> = Vec::new();
    if args.script_file_name.len() > 0 {
        script = read_forth_source(&args.script_file_name)?;
    }

    // read a j1 binary file
    let mut binary: Vec<u8> = Vec::new();
    if args.bin_file_name.len() == 0 {
        println!("loaded j1e binary");
        binary = j1e_bin::J1E_BIN.to_vec();
    } else {
        println!("loaded binary: {}", args.bin_file_name);
        binary = read_binary(&args.bin_file_name)?;
    }

    // make a j1 cpu and run the script
    let mut done = false;
    let mut cpu = CPU::new();
    cpu.load_bytes(&binary)?;
    cpu.run(script).or_else(|e| {
        if e == "bye" {
            done = true;
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, e))
        }
    })?;

    if args.repl {
        print!("{}", cpu.console.get_log());
        cpu.console.writer.clear();
        while !done {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.len() > 0 {
                cpu.run(Vec::from(input)).or_else(|e| {
                    if e == "bye" {
                        done = true;
                        Ok(())
                    } else {
                        Err(Error::new(ErrorKind::Other, e))
                    }
                })?;
                print!("{}", cpu.console.get_writer());
                cpu.console.writer.clear();
            }
        }
    } else {
        let log_file_name = args.script_file_name.clone() + "-log.txt";
        write_log_file(&log_file_name, &cpu.console.get_log())?;
        println!("log written to: {}", &log_file_name);
        // println!("{}", cpu.console.get_log());
        println!("\nExiting j1...");
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
pub struct Args {
    pub bin_file_name: String,
    pub script_file_name: String,
    pub repl: bool,
}

#[allow(dead_code)]
impl Args {
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
            .help("Binary J1 forth image to load, default is j1e.bin")
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
        Ok(Args { bin_file_name, script_file_name, repl })
    }
}
