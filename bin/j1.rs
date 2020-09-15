use std::io::{Error, ErrorKind};

use j1::cpu::CPU;
use j1::j1_args::J1Args;
use j1::j1e_bin;
use j1::utils::{read_binary, read_forth_source, write_log_file};
use std::io;

#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    println!("Starting j1...\n");

    let j1_args = J1Args::new();
    // println!("{:?}", j1_args);
    if !j1_args.repl && j1_args.script_file_name.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "Must provide a script file if not running repl"))
    }

    // read a forth script file
    let mut script: Vec<u8> = Vec::new();
    if j1_args.script_file_name.len() > 0 {
        script = read_forth_source(&j1_args.script_file_name)?;
    }

    // read a j1 binary file
    let mut binary: Vec<u8> = Vec::new();
    if j1_args.bin_file_name.len() == 0 {
        println!("loaded j1e binary");
        binary = j1e_bin::J1E_BIN.to_vec();
    } else {
        println!("loaded binary: {}", j1_args.bin_file_name);
        binary = read_binary(&j1_args.bin_file_name)?;
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

    if j1_args.repl {
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
        let log_file_name = j1_args.script_file_name.clone() + "-log.txt";
        write_log_file(&log_file_name, &cpu.console.get_log())?;
        println!("log written to: {}", &log_file_name);
        // println!("{}", cpu.console.get_log());
        println!("\nExiting j1...");
    }
    Ok(())
}

