use std::io::{Error, ErrorKind};

use j1::cpu::CPU;
use j1::j1_dump_args::J1DumpArgs;
use j1::j1e_bin;
use j1::utils::{read_binary, read_forth_source};

#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    let j1_dump_args = J1DumpArgs::new();
    // println!("{:?}", j1_dump_args);

    // read a forth script file
    let mut script: Vec<u8> = Vec::new();
    if j1_dump_args.script_file_name.len() > 0 {
        script = read_forth_source(&j1_dump_args.script_file_name)?;
    }

    // read a j1 binary file
    let mut binary: Vec<u8> = Vec::new();
    if j1_dump_args.bin_file_name.len() == 0 {
        println!("loaded j1e binary");
        binary = j1e_bin::J1E_BIN.to_vec();
    } else {
        println!("loaded binary: {}", j1_dump_args.bin_file_name);
        binary = read_binary(&j1_dump_args.bin_file_name)?;
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

    let xs = cpu.dump_asm(j1_dump_args.addr_start, j1_dump_args.addr_end);
    xs.iter().for_each(|x| println!("{}", x));

    Ok(())
}

