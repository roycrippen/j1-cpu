use j1::cpu::CPU;
use j1::j1e_bin;

#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    let addr_start = 0x1A04;
    let addr_end = 0x1A54;

    let script = ": fibonacci dup 2 < if drop 1 else dup 2 - recurse swap 1 - recurse + then ; \n";
    println!("Forth script to compile and dump from j1e => `{}`", script.trim());

    let mut cpu = CPU::new();
    cpu.load_bytes(&j1e_bin::J1E_BIN.to_vec()).unwrap();
    let xs = cpu.dump_asm(addr_start, addr_end);
    println!("\nmemory ASM from 0x{:04X} to 0x{:04X} before compile script: ", addr_start, addr_end);
    xs.iter().for_each(|x| println!("{}", x));

    cpu.run(script.bytes().collect()).unwrap();
    println!("\nmemory ASM from 0x{:04X} to 0x{:04X} after compile script: ", addr_start, addr_end);
    let xs = cpu.dump_asm(addr_start, addr_end);
    xs.iter().for_each(|x| println!("{}", x));

    println!("\nmemory AST from 0x{:04X} to 0x{:04X} after compile script: ", addr_start, addr_end);
    let xs = cpu.dump_ast(addr_start, addr_end);
    xs.iter().for_each(|x| println!("{}", x));

    Ok(())
}
