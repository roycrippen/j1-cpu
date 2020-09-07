use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::console::{Console, IO, MockConsole};
use crate::stack::Stack;
use crate::instruction::{Instruction, decode, OpCode};

const IO_MASK: u16 = 3 << 14;

#[allow(dead_code)]
pub struct CPU<T: IO> {
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
impl<T: IO> CPU<T> {
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

    fn reset(&mut self) {
        self.pc = 0;
        self.st0 = 0;
        self.d = Stack::default();
        self.r = Stack::default();
    }

    fn read_at(&mut self, addr: u16) -> u16 {
        if addr & IO_MASK == 0 {
            return self.memory[addr as usize];
        }
        match addr {
            0x7000 => self.console.buf.read_byte().unwrap_or(0) as u16, // tx!
            0x7001 => self.console.buf.buf_len() as u16,                // ?rx
            _ => 0 as u16 // error
        }
    }

    fn fetch(&self) -> Result<Instruction, String> {
        decode(self.memory[self.pc as usize])
    }

    fn execute(&mut self, _ins: &Instruction) -> Result<(), String> {
        Ok(())
    }

    fn run(&mut self) {
        loop {
            if let Ok(ins) = self.fetch() {
                if let Err(e) = self.execute(&ins) {
                    if e == "bye" {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    fn new_st0(&mut self, opcode: &OpCode) -> u16 {
        let bool_value = |b: bool| -> u16 { if b { !0 } else { 0 } };
        let t = self.st0;       // T
        let n = self.d.peek();  // N
        let r = self.r.peek();  // R
        match opcode {
            OpCode::OpT => t,                      // T
            OpCode::OpN => n,                      // N
            OpCode::OpTplusN => t + n,             // T + N
            OpCode::OpTandN => t & n,              // T & N
            OpCode::OpTorN => t | n,               // T | N
            OpCode::OpTxorN => t ^ n,              // T ^ N
            OpCode::OpNotT => !t,                  //  !T
            OpCode::OpNeqT => bool_value(n == t),  // N == T
            OpCode::OpNleT => bool_value((n as i16) < (t as i16)),   // N < T
            OpCode::OpNrshiftT => n >> (t & 0xf),  // N >> T
            OpCode::OpTminus1 => t - 1,            // T - 1
            OpCode::OpR => r,                      // R
            OpCode::OpAtT => self.read_at(t),      // [T]
            OpCode::OpNlshiftT => n << (t & 0xf),  // N << T
            OpCode::OpDepth => (self.r.depth() << 8) | self.d.depth(),  // depth (dsp)
            OpCode::OpNuleT => bool_value(n < t),  // Nu < T
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
        let mut f = File::open(file_name).expect("Can not find binary file");
        let xs = &mut Vec::new();
        f.read_to_end(xs).expect("Read file failed");
        self.load_bytes(xs)?;

        Ok(())
    }
}

// helpers
pub fn load_j1e_bin() -> CPU<MockConsole> {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("resources");
    p.push("j1e.bin");
    let full_file_name = p.display().to_string();

    let console = Console::<MockConsole>::new(true);
    let mut cpu = CPU::new(console);
    cpu.load_bytes_from_file(full_file_name).unwrap();
    cpu
}

#[cfg(test)]
mod tests {
    use crate::console::{Console, MockConsole, IO};
    use crate::cpu::{CPU, load_j1e_bin};
    use crate::instruction::OpCode::*;

    #[test]
    fn reset() {
        let console: Console<MockConsole> = Console::new(true);
        let mut cpu = CPU::new(console);

        cpu.pc = 100;
        cpu.st0 = 10;
        cpu.d.move_sp(10);
        cpu.r.move_sp(20);
        debug_assert_eq!(140, cpu.pc + cpu.st0 + cpu.d.depth() + cpu.r.depth());

        cpu.reset();
        debug_assert_eq!(0, cpu.pc + cpu.st0 + cpu.d.depth() + cpu.r.depth());
    }

    #[test]
    fn reaad_at() {
        let mut cpu = load_j1e_bin();
        assert_eq!(16128, cpu.read_at(5));
        assert_eq!(3650, cpu.read_at(6));

        let mut cmds: Vec<u8> = "1 2 + .s\n".bytes().collect();
        cpu.console.buf.load_buf(&mut cmds);

        // 0x7000 => tx!,  0x7001 => ?rx
        assert_eq!(9, cpu.read_at(0x7001));
        assert_eq!('1' as u16, cpu.read_at(0x7000));
        assert_eq!(' ' as u16, cpu.read_at(0x7000));
        assert_eq!('2' as u16, cpu.read_at(0x7000));
        assert_eq!(6, cpu.read_at(0x7001));
    }

    #[test]
    fn load_bytes() {
        let console = Console::<MockConsole>::new(true);
        let mut cpu = CPU::new(console);

        let data = &mut vec![1, 2, 4, 8];
        cpu.load_bytes(data).unwrap();

        let xs = &cpu.memory[0..2];
        assert_eq!([0x0201, 0x0804], xs);
        // println!("first {} items memory: {:?}", xs.len(), xs);
    }

    #[test]
    fn load_bytes_from_file() {
        let cpu = load_j1e_bin();
        let xs = &cpu.memory[0..8];
        assert_eq!([3306, 16, 0, 0, 0, 16128, 3650, 3872], xs);
        // println!("first {} items memory: {:?}", xs.len(), xs);
    }

    #[test]
    fn new_st0() {
        let mut cpu = load_j1e_bin();

        let test_cases = [
            (OpN, 66u16, 16000u16, 6620u16, 16000u16),
            (OpR, 16000, 2, 66, 66),
            (OpTminus1, 66, 16000, 6620, 65),
            (OpT, 2, 0, 16000, 2),
            (OpTandN, 1, 2, 2774, 0),
            (OpNlshiftT, 8, 16, 2778, 4096),
            (OpTorN, 4096, 16, 2778, 4112),
            (OpNeqT, 0, 0, 2778, 65535),
            (OpTxorN, 255, 65535, 2778, 65280),
            (OpTplusN, 1, 2, 2780, 3),
            (OpNlshiftT, 8, 16, 2778, 4096),
            (OpNotT, 2, 16386, 1326, 65533),
            (OpNuleT, 2, 0, 1892, 65535),
            (OpNleT, 0, 0, 878, 0),
            (OpAtT, 2, 0, 0, 0),
        ];
        for (opcode, t, n, r, expected) in test_cases.iter() {
            cpu.st0 = *t;
            cpu.d.push(*n);
            cpu.r.push(*r);
            assert_eq!(*expected, cpu.new_st0(opcode));
            // println!("opcode {:?}, t {}, n {}, t {}", opcode, t, n, r);
            cpu.d.pop();
            cpu.r.pop();
        }

        // OpDepth check
        cpu.st0 = 0;
        cpu.d.move_sp(0);
        cpu.r.move_sp(4);
        assert_eq!(1024, cpu.new_st0(&OpDepth));
        // println!("{:?}, d.depth() = {}, r.depth() = {}", OpDepth, cpu.d.depth(), cpu.r.depth());
    }
}