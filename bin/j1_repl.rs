use std::io::{Error, ErrorKind};
use std::io;

use j1::console::*;
use j1::cpu::CPU;
use j1::j1e_bin;
use j1::utils::read_binary;

#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    println!("Starting j1...");

    let bin_file_name = "".to_string();
    // let bin_file_name = "/home/roy/dev/rust/j1-cpu/resources/j1e.bin".to_string();

    // read a j1 binary file
    let mut binary: Vec<u8> = Vec::new();
    if bin_file_name.len() == 0 {
        println!("loaded j1e binary");
        binary = j1e_bin::J1E_BIN.to_vec();
    } else {
        println!("loaded binary: {}", bin_file_name);
        binary = read_binary(&bin_file_name)?;
    }

    // make a j1 cpu
    let stdio = io::stdin();
    let input = stdio.lock();
    let console = Console { reader: input, writer: io::stdout(), log: Vec::new() };
    let mut cpu = CPU::new(console);
    cpu.load_bytes(&binary)?;

    match cpu.run() {
        Ok(_) => Ok(()),
        Err(e) => {
            if e == "bye" {
                println!("\nExiting J1e repl");
                Ok(())
            } else {
                Err(Error::new(ErrorKind::Other, e))
            }
        }
    }
}

