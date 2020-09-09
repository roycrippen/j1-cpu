use j1::console::*;
use j1::j1e_bin;
use j1::cpu::CPU;
use j1::utils::{read_binary, read_forth_source, write_log_file};
use std::io::{ErrorKind, Error};

#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    println!("Starting j1...\n");

    let bin_file_name = "".to_string();
    // let bin_file_name = "/home/roy/dev/rust/j1-cpu/resources/j1e.bin".to_string();
    let script_file_name = "/home/roy/dev/rust/j1-cpu/resources/simple.fth".to_string();

    // read a forth script file
    let mut script: Vec<u8> = Vec::new();
    if script_file_name.len() > 0 {
        script = read_forth_source(&script_file_name)?;
    }
    script.append(&mut "\nbye\n".as_bytes().to_vec());

    // read a j1 binary file
    let mut binary:  Vec<u8> = Vec::new();
    if bin_file_name.len() == 0 {
        println!("loaded j1e binary");
        binary = j1e_bin::J1E_BIN.to_vec();
    } else {
        println!("loaded binary: {}", bin_file_name);
        binary = read_binary(&bin_file_name)?;
    }

    // make a j1 cpu
    let console: Console<MockConsole> = Console::new(true);
    let mut cpu = CPU::new(console);
    cpu.load_bytes(&binary)?;

    match cpu.run(script) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e == "bye" {
                let name = script_file_name.clone() + "-log.txt";
                write_log_file(&name, &cpu.console.get_log())?;
                println!("log written to: {}", &name);
                // println!("{:?}", cpu.console.get_log());
                println!("\nExiting j1...");
                Ok(())
            } else {
                Err(Error::new(ErrorKind::Other, e))
            }
        }
    }
}

