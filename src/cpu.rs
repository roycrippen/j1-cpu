use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::console::Console;
use crate::stack::Stack;

#[allow(dead_code)]
pub struct CPU<T: Read + Write> {
    // 0..0x3fff RAM, 0x4000..0x7fff mem-mapped I/O
    memory: Box<[u16; 8192]>,

    // 13 bit program counter
    pc: u16,

    // top of data stack
    st0: u16,

    // 32 deep × 16 bit data stack
    d: Stack,

    // 32 deep × 16 bit return stacks
    r: Stack,

    // io console
    console: Console<T>,
}

#[allow(dead_code)]
impl<T: Read + Write> CPU<T> {
    fn new(console: Console<T>) -> Self {
        CPU {
            memory: Box::new([0u16; 8192]),
            pc: 0,
            st0: 0,
            d: Stack::default(),
            r: Stack::default(),
            console,
        }
    }

    fn load_bytes(&mut self, data: &mut Vec<u8>) -> Result<(), String> {
        if data.len() % 2 != 0 {
            return Err("Odd number of bytes provided".to_string());
        }

        let size = data.len() >> 1;
        if size >= self.memory.len() {
            return Err("Binary too big for cpu memory to load".to_string());
        }

        let mut current = &data[..];
        let mut i = 0;
        while current.len() > 0 {
            self.memory[i] = current.read_u16::<LittleEndian>().expect("Could not convert binary");
            i += 1;
        }
        Ok(())
    }

    fn load_bytes_from_file(&mut self, file_name: String) -> Result<(), String> {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("resources");
        p.push(file_name);
        let full_file_name = p.display().to_string();
        let mut f = File::open(full_file_name).expect("Can not find binary file");
        let xs = &mut Vec::new();
        f.read_to_end(xs).expect("Read file failed");
        self.load_bytes(xs)?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::console::Console;
    use crate::cpu::CPU;

    #[test]
    fn load_bytes() {
        let console = Console { ar1: [0], in_buff: Cursor::new(Vec::new()), out_buff: Cursor::new(Vec::new()) };
        let mut cpu = CPU::new(console);

        let data = &mut vec![234, 12, 16, 0, 0, 0];
        cpu.load_bytes(data).unwrap();

        let xs = &cpu.memory[0..3];
        assert_eq!([3306, 16, 0], xs);
        assert_eq!(0, xs[2]);
        // println!("first {} items memory: {:?}", xs.len(), xs);
    }

    #[test]
    fn load_bytes_from_file() {
        let console = Console { ar1: [0], in_buff: Cursor::new(Vec::new()), out_buff: Cursor::new(Vec::new()) };
        let mut cpu = CPU::new(console);
        cpu.load_bytes_from_file("j1e.bin".to_string()).unwrap();

        let xs = &cpu.memory[0..8];
        assert_eq!([3306, 16, 0, 0, 0, 16128, 3650, 3872], xs);
        // println!("first {} items memory: {:?}", xs.len(), xs);
    }
}