use j1::console::{Console, MockConsole};
use j1::cpu::CPU;
use j1::j1e_bin;

fn main() -> Result<(), String> {
    println!("Start J1e repl...");

    let console: Console<MockConsole> = Console::new(true);
    let mut cpu = CPU::new(console);
    cpu.load_bytes(&mut j1e_bin::J1E_BIN.to_vec())?;
    match cpu.run() {
        Ok(_) => Ok(()),
        Err(e) => {
            if e == "bye" {
                println!("\nExiting J1e repl");
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

