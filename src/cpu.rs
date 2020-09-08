use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::console::{Console, IO, MockConsole};
use crate::instruction::{decode, Instruction, OpCode};
use crate::instruction::Instruction::{ALU, Call, Conditional, Jump, Literal};
use crate::stack::Stack;

const IO_MASK: u16 = 3 << 14;

#[allow(dead_code)]
#[derive(Clone)]
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
    pub fn new(console: Console<T>) -> Self {
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

    fn write_at(&mut self, addr: u16, value: u16) -> Result<(), String> {
        if addr & IO_MASK == 0 {
            self.memory[(addr >> 1) as usize] = value;
        }
        match addr {
            0x7000 => self.console.buf.write_byte(value as u8),  // key
            0x7001 => return Err("bye".to_string()),             // bye
            _ => ()
        }
        Ok(())
    }

    fn read_at(&mut self, addr: u16) -> u16 {
        if addr & IO_MASK == 0 {
            return self.memory[(addr >> 1) as usize];
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

    fn execute(&mut self, ins: &Instruction) -> Result<(), String> {
        self.pc += 1;
        match ins {
            Literal(v) => {
                self.d.push(self.st0);
                self.st0 = *v
            }
            Jump(v) => self.pc = *v,
            Call(v) => {
                self.r.push(self.pc << 1);
                self.pc = *v
            }
            Conditional(v) => {
                if self.st0 == 0 {
                    self.pc = *v
                }
                self.st0 = self.d.pop()
            }
            ALU(alu) => {
                if alu.r2pc {
                    self.pc = self.r.peek() >> 1
                }
                if alu.n2_at_t {
                    self.write_at(self.st0, self.d.peek())?;
                }
                let st0 = self.new_st0(&alu.opcode);
                self.d.move_sp(alu.d_dir);
                self.r.move_sp(alu.r_dir);
                if alu.t2n {
                    self.d.replace(self.st0)
                }
                if alu.t2r {
                    self.r.replace(self.st0)
                }
                self.st0 = st0
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        // let mut cnt = 0;
        loop {
            let ins = self.fetch().or_else(|e| Err(e))?;
            // if cnt <= 100 {
            //     cnt += 1;
            //     println!("cnt = {:<6} => {}", cnt, ins.show());
            // }
            self.execute(&ins)?
        }
    }

    fn new_st0(&mut self, opcode: &OpCode) -> u16 {
        let bool_value = |b: bool| -> u16 { if b { !0 } else { 0 } };
        let t = self.st0;       // T
        let n = self.d.peek();  // N
        let r = self.r.peek();  // R
        match opcode {
            OpCode::OpT => t,                                           // T
            OpCode::OpN => n,                                           // N
            OpCode::OpTplusN => t.wrapping_add(n),                      // T + N
            OpCode::OpTandN => t & n,                                   // T & N
            OpCode::OpTorN => t | n,                                    // T | N
            OpCode::OpTxorN => t ^ n,                                   // T ^ N
            OpCode::OpNotT => !t,                                       //  !T
            OpCode::OpNeqT => bool_value(n == t),                       // N == T
            OpCode::OpNleT => bool_value((n as i16) < (t as i16)),      // N < T
            OpCode::OpNrshiftT => n >> (t & 0xf),                       // N >> T
            OpCode::OpTminus1 => t - 1,                                 // T - 1
            OpCode::OpR => r,                                           // R
            OpCode::OpAtT => self.read_at(t),                           // [T]
            OpCode::OpNlshiftT => n << (t & 0xf),                       // N << T
            OpCode::OpDepth => (self.r.depth() << 8) | self.d.depth(),  // depth (dsp)
            OpCode::OpNuleT => bool_value(n < t),                       // Nu < T
        }
    }

    pub fn load_bytes(&mut self, data: &mut Vec<u8>) -> Result<(), String> {
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

    pub fn load_bytes_from_file(&mut self, file_name: String) -> Result<(), String> {
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
    use crate::console::{Console, IO, MockConsole};
    use crate::cpu::{CPU, load_j1e_bin};
    use crate::instruction::OpCode::*;
    use crate::instruction::{Instruction, AluAttributes, OpCode};
    use crate::instruction::Instruction::{Jump, Literal, Conditional, Call, ALU};

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
        assert_eq!(16128, cpu.read_at(11));
        assert_eq!(3650, cpu.read_at(12));

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
    fn new_st0_1() {
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
            (OpAtT, 2, 0, 0, 16),
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

    #[test]
    fn new_st0_2() {
        let console = Console::<MockConsole>::new(true);
        let default_cpu = CPU::new(console);
        let mut test_cases: Vec<(OpCode, u16, CPU<MockConsole>)> = vec![];

        let mut cpu = default_cpu.clone();
        cpu.st0 = 0x55;
        test_cases.push((OpT, 0x55, cpu.clone()));
        test_cases.push((OpTminus1, 0x54, cpu.clone()));
        test_cases.push((OpNotT, 0xffaa, cpu.clone()));

        cpu = default_cpu.clone();
        cpu.st0 = 0xff;
        cpu.d.push(0xaa);
        cpu.d.push(0xbb);
        test_cases.push((OpN, 0xbb, cpu.clone()));
        test_cases.push((OpTplusN, 0x01ba, cpu.clone()));
        test_cases.push((OpTandN, 0xbb, cpu.clone()));
        test_cases.push((OpTorN, 0xff, cpu.clone()));
        test_cases.push((OpTxorN, 0x44, cpu.clone()));
        test_cases.push((OpNeqT, 0x00, cpu.clone()));
        test_cases.push((OpNleT, 0xffff, cpu.clone()));
        test_cases.push((OpNuleT, 0xffff, cpu.clone()));

        cpu = default_cpu.clone();
        cpu.st0 = 0xff;
        cpu.d.push(0xaa);
        cpu.d.push(0xff);
        test_cases.push((OpNeqT, 0xffff, cpu.clone()));
        test_cases.push((OpNleT, 0x00, cpu.clone()));
        test_cases.push((OpNuleT, 0x00, cpu.clone()));

        cpu = default_cpu.clone();
        cpu.st0 = 0x02;
        cpu.d.push(0xaa);
        cpu.d.push(0xff);
        test_cases.push((OpNrshiftT, 0x3f, cpu.clone()));
        test_cases.push((OpNlshiftT, 0x3fc, cpu.clone()));

        cpu = default_cpu.clone();
        cpu.r.push(0x05);
        test_cases.push((OpR, 0x5, cpu.clone()));

        cpu = default_cpu.clone();
        cpu.st0 = 0x02;
        cpu.memory[0] = 0;
        cpu.memory[1] = 5;
        cpu.memory[2] = 10;
        test_cases.push((OpAtT, 0x5, cpu.clone()));


        for (opcode, expected_st0, cpu) in test_cases.iter() {
            let st0 = cpu.clone().new_st0(opcode);
            assert_eq!(*expected_st0, st0)
        }
    }

    #[test]
    fn eval() {
        let cmp = |expected: &CPU<MockConsole>, result: &CPU<MockConsole>| {
            assert_eq!(expected.pc, result.pc);
            assert_eq!(expected.st0, result.st0);
            assert_eq!(expected.d.sp, result.d.sp);
            assert_eq!(expected.r.sp, result.r.sp);
            assert_eq!(expected.d.dump(), result.d.dump());
            assert_eq!(expected.r.dump(), result.r.dump());
        };
        let console = Console::<MockConsole>::new(true);
        let default_cpu = CPU::new(console);

        struct Eval { inss: Vec<Instruction>, e_cpu: CPU<MockConsole> }
        let mut test_cases: Vec<Eval> = vec![];

        // test 01
        let mut inss = vec![Jump(0xff)];
        let mut e_cpu = default_cpu.clone();
        e_cpu.pc = 0xff;
        test_cases.push(Eval { inss, e_cpu });

        // test 02
        inss = vec![Literal(1), Conditional(0xff)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 2;
        test_cases.push(Eval { inss, e_cpu });

        // test 03
        inss = vec![Literal(0), Conditional(0xff)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 0xff;
        test_cases.push(Eval { inss, e_cpu });

        // test 04
        inss = vec![Call(0xff)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 0xff;
        e_cpu.r.push(0x02);
        test_cases.push(Eval { inss, e_cpu });

        // test 05
        inss = vec![Literal(0xff)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 1;
        e_cpu.st0 = 0xff;
        e_cpu.d.sp = 1;
        test_cases.push(Eval { inss, e_cpu });

        // test 06
        inss = vec![Literal(0xff), Literal(0xfe)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 2;
        e_cpu.st0 = 0xfe;
        e_cpu.d.push(0x00);
        e_cpu.d.push(0xff);
        test_cases.push(Eval { inss, e_cpu });

        // test 07 - dup
        let mut alu = AluAttributes::default();
        alu.opcode = OpT;
        alu.t2n = true;
        alu.d_dir = 1;
        inss = vec![Literal(0xff), ALU(alu)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 2;
        e_cpu.st0 = 0xff;
        e_cpu.d.push(0x00);
        e_cpu.d.push(0xff);
        test_cases.push(Eval { inss, e_cpu });

        // test 08 - over
        alu = AluAttributes::default();
        alu.opcode = OpN;
        alu.t2n = true;
        alu.d_dir = 1;
        inss = vec![Literal(0xaa), Literal(0xbb), ALU(alu)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 3;
        e_cpu.st0 = 0xaa;
        e_cpu.d.push(0x00);
        e_cpu.d.push(0xaa);
        e_cpu.d.push(0xbb);
        test_cases.push(Eval { inss, e_cpu });

        // test 09 - invert
        alu = AluAttributes::default();
        alu.opcode = OpNotT;
        inss = vec![Literal(0x00ff), ALU(alu)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 2;
        e_cpu.st0 = 0xff00;
        e_cpu.d.sp = 1;
        test_cases.push(Eval { inss, e_cpu });

        // test 10 - plus
        alu = AluAttributes::default();
        alu.opcode = OpTplusN;
        alu.d_dir = -1;
        inss = vec![Literal(1), Literal(2), ALU(alu)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 3;
        e_cpu.st0 = 3;
        e_cpu.d.push(0);
        e_cpu.d.push(1);
        e_cpu.d.sp = 1;
        test_cases.push(Eval { inss, e_cpu });

        // test 11 - swap
        alu = AluAttributes::default();
        alu.opcode = OpN;
        alu.t2n = true;
        inss = vec![Literal(2), Literal(3), ALU(alu)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 3;
        e_cpu.st0 = 2;
        e_cpu.d.push(0);
        e_cpu.d.push(3);
        e_cpu.d.sp = 2;
        test_cases.push(Eval { inss, e_cpu });

        // test 12 - nip
        alu = AluAttributes::default();
        alu.opcode = OpT;
        alu.d_dir = -1;
        inss = vec![Literal(2), Literal(3), ALU(alu)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 3;
        e_cpu.st0 = 3;
        e_cpu.d.push(0);
        e_cpu.d.push(2);
        e_cpu.d.sp = 1;
        test_cases.push(Eval { inss, e_cpu });

        // test 13 - drop
        alu = AluAttributes::default();
        alu.opcode = OpN;
        alu.d_dir = -1;
        inss = vec![Literal(2), Literal(3), ALU(alu)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 3;
        e_cpu.st0 = 2;
        e_cpu.d.push(0);
        e_cpu.d.push(2);
        e_cpu.d.sp = 1;
        test_cases.push(Eval { inss, e_cpu });

        // test 14 - ;
        alu = AluAttributes::default();
        alu.opcode = OpT;
        alu.r_dir = -1;
        alu.r2pc = true;
        inss = vec![Call(10), Call(20), ALU(alu)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 11;
        e_cpu.r.push(2);
        e_cpu.r.push(22);
        e_cpu.r.sp = 1;
        test_cases.push(Eval { inss, e_cpu });

        // test 15 - >r
        alu = AluAttributes::default();
        alu.opcode = OpN;
        alu.r_dir = 1;
        alu.d_dir = -1;
        alu.t2r = true;
        inss = vec![Literal(10), ALU(alu)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 2;
        e_cpu.r.push(10);
        e_cpu.r.sp = 1;
        test_cases.push(Eval { inss, e_cpu });

        // test 16 - r>
        alu = AluAttributes::default();
        alu.opcode = OpR;
        alu.r_dir = -1;
        alu.d_dir = 1;
        alu.t2n = true;
        inss = vec![Literal(10), Call(20), ALU(alu)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 21;
        e_cpu.st0 = 4;
        e_cpu.d.push(0);
        e_cpu.d.push(10);
        e_cpu.d.sp = 2;
        e_cpu.r.push(10);
        e_cpu.r.push(4);
        e_cpu.r.sp = 0;
        test_cases.push(Eval { inss, e_cpu });

        // test 17 - r@
        alu = AluAttributes::default();
        alu.opcode = OpR;
        alu.r_dir = 0;
        alu.d_dir = 1;
        alu.t2n = true;
        inss = vec![Literal(10), ALU(alu)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 2;
        e_cpu.st0 = 0;
        e_cpu.d.push(0);
        e_cpu.d.push(10);
        e_cpu.d.sp = 2;
        e_cpu.r.push(10);
        e_cpu.r.sp = 0;
        test_cases.push(Eval { inss, e_cpu });

        // test 18 - r@
        alu = AluAttributes::default();
        alu.opcode = OpAtT;
        inss = vec![ALU(alu)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 1;
        test_cases.push(Eval { inss, e_cpu });

        // test 19 - !
        alu = AluAttributes::default();
        alu.opcode = OpN;
        alu.d_dir = -1;
        alu.n2_at_t = true;
        inss = vec![Literal(1), Literal(0), ALU(alu)];
        e_cpu = default_cpu.clone();
        e_cpu.pc = 3;
        e_cpu.st0 = 1;
        e_cpu.d.push(0);
        e_cpu.d.push(1);
        e_cpu.d.sp = 1;
        e_cpu.memory[0] = 1;
        test_cases.push(Eval { inss, e_cpu });

        for s in test_cases.iter() {
            let mut cpu = default_cpu.clone();
            for ins in &s.inss {
                let _ = cpu.execute(&ins);
            }
            cmp(&s.e_cpu, &cpu);
        }
    }
}