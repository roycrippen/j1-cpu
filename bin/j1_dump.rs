extern crate clap;

use std::ffi::OsString;
use std::io::{Error, ErrorKind};

use clap::{App, Arg};

use j1::cpu::CPU;
use j1::j1e_bin;
use j1::utils::{read_binary, read_forth_source};

#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    let args = Args::new();
    // println!("{:?}", args);

    // read a forth script file
    let mut script: Vec<u8> = Vec::new();
    if args.script_file_name.len() > 0 {
        script = read_forth_source(&args.script_file_name)?;
    }

    // read a j1 binary file
    let mut binary: Vec<u8> = Vec::new();
    if args.bin_file_name.len() == 0 {
        binary = j1e_bin::J1E_BIN.to_vec();
    } else {
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

    let mut xs = Vec::new();
    if args.ast {
        xs = cpu.dump_ast(args.addr_start, args.addr_end);
    } else {
        xs = cpu.dump_asm(args.addr_start, args.addr_end);
    }
    xs.iter().for_each(|x| println!("{}", x));

    Ok(())
}

#[derive(Debug, PartialEq)]
pub struct Args {
    pub bin_file_name: String,
    pub script_file_name: String,
    pub addr_start: u16,
    pub addr_end: u16,
    pub ast: bool
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
            .about("J1 cpu memory dump")
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


        // define ast name command line flag
        let ast_flag = Arg::with_name("ast")
            .long("ast") // allow --repl
            .help("Dump Abstract Syntax Tree of instructions (instead of assembly")
            .required(false);

        let app = app
            .arg(bin_file_name_option)
            .arg(script_file_name_option)
            .arg(addr_start_option)
            .arg(addr_end_option)
            .arg(ast_flag);

        let matches = app.get_matches_from_safe(args)?;
        let bin_file_name = matches.value_of("bin_file").unwrap_or("").to_string();
        let script_file_name = matches.value_of("script_file").unwrap_or("").to_string();

        let err = clap::Error::with_description("Invalid address end", clap::ErrorKind::InvalidValue);
        let raw = matches.value_of("addr_start").unwrap();
        let without_prefix = raw.trim_start_matches("0x");
        let addr_start = u16::from_str_radix(without_prefix, 16).or_else(|_e| Err(&err)).unwrap();

        let raw = matches.value_of("addr_end").unwrap();
        let without_prefix = raw.trim_start_matches("0x");
        let addr_end = u16::from_str_radix(without_prefix, 16).or_else(|_e| Err(&err)).unwrap();

        if addr_start > addr_end || addr_end > 0x2000 {
            return Err(clap::Error::with_description("Invalid addresses", clap::ErrorKind::InvalidValue));
        }

        let mut ast = false;
        if matches.occurrences_of("ast") > 0 {
            ast = true
        }
         Ok(Args { bin_file_name, script_file_name, addr_start, addr_end, ast })
    }
}
