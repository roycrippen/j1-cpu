use j1::console::*;
use j1::j1e_bin;
use j1::cpu::CPU;
use std::process::Stdio;
use j1::utils::{read_binary, read_forth_source};
use std::io::{ErrorKind, Error};

#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    println!("Starting j1...");

    let bin_file_name = "".to_string();
    let script_file_name = "".to_string();

    // read a forth script file
    let mut script: Vec<u8> = Vec::new();
    if script_file_name.len() > 0 {
        script = read_forth_source(script_file_name)?;
    }

    // read a j1 binary file
    let mut binary:  Vec<u8> = Vec::new();
    if bin_file_name.len() > 0 {
        println!("loaded j1e binary");
        binary = j1e_bin::J1E_BIN.to_vec();
    } else {
        println!("loaded binary: {}", bin_file_name);
        binary = read_binary(bin_file_name)?;
    }

    // make a j1 cpu
    let console: Console<Stdio> = Console::new(false);
    let mut cpu = CPU::new(console);
    cpu.load_bytes(&binary)?;

    match cpu.run(script) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e == "bye" {
                println!("\nExiting J1e repl");
                println!("{:?}", cpu.console.get_log());
                Ok(())
            } else {
                Err(Error::new(ErrorKind::Other, e))
            }
        }
    }
}

