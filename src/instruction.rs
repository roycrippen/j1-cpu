use std::fmt;
use crate::instruction::Instruction::*;

const EXPAND: [i8; 4] = [0, 1, -2, -1];
const OPCODE_NAMES: [&'static str; 16] = [
    "T", "N", "T+N", "T∧N", "T∨N", "T⊻N", "¬T", "N=T",
    "N<T", "N≫T", "T-1", "R", "[T]", "N≪T", "D", "Nu<T"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    // Literal value
    //
    //  15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    //   │  └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴── value
    //   └─────────────────────────────────────────────── 1
    //
    Literal(u16),

    // Jump instruction
    //
    //  15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    //   │  │  │  └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴── target
    //   └──┴──┴───────────────────────────────────────── 0 0 0
    //
    Jump(u16),

    // Conditional jump instruction
    //
    //  15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    //   │  │  │  └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴── target
    //   └──┴──┴───────────────────────────────────────── 0 0 1
    //
    Conditional(u16),

    // Call instruction
    //
    //  15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    //   │  │  │  └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴── target
    //   └──┴──┴───────────────────────────────────────── 0 1 0
    //
    Call(u16),

    // ALU instruction
    //
    //  15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    //   │  │  │  │  │  │  │  │  │  │  │  │  │  │  └──┴── dstack ±
    //   │  │  │  │  │  │  │  │  │  │  │  │  └──┴──────── rstack ±
    //   │  │  │  │  │  │  │  │  │  │  │  └────────────── unused
    //   │  │  │  │  │  │  │  │  │  │  └───────────────── N → [T]
    //   │  │  │  │  │  │  │  │  │  └──────────────────── T → R
    //   │  │  │  │  │  │  │  │  └─────────────────────── T → N
    //   │  │  │  │  └──┴──┴──┴────────────────────────── Tʹ
    //   │  │  │  └────────────────────────────────────── R → PC
    //   └──┴──┴───────────────────────────────────────── 0 1 1
    //
    ALU(AluAttributes),
}

pub fn decode(v: u16) -> Result<Instruction, String> {
    match v {
        v if v & (1 << 15) == 1 << 15 => Ok(Literal(v & !(1 << 15))),
        v if v & (7 << 13) == 0 => Ok(Jump(v & !(7 << 13))),
        v if v & (7 << 13) == 1 << 13 => Ok(Conditional(v & !(7 << 13))),
        v if v & (7 << 13) == 2 << 13 => Ok(Call(v & !(7 << 13))),
        v if v & (7 << 13) == 3 << 13 => Ok(ALU(decode_alu(v))),
        _ => Err(format!("Invalid Instruction: {:0>4x}", v)),
    }
}

pub fn decode_alu(v: u16) -> AluAttributes {
    AluAttributes {
        opcode: (v >> 8) & 15,
        r2pc: v & (1 << 12) != 0,
        t2n: v & (1 << 7) != 0,
        t2r: v & (1 << 6) != 0,
        n2t: v & (1 << 5) != 0,
        r_dir: EXPAND[((v >> 2) & 3) as usize],
        d_dir: EXPAND[((v >> 0) & 3) as usize],
    }
}

impl Instruction {
    pub fn value(&self) -> u16 {
        match self {
            Literal(v) => *v,
            Jump(v) => *v,
            Conditional(v) => *v,
            Call(v) => *v,
            ALU(alu) => alu.value()
        }
    }

    pub fn compile(&self) -> u16 {
        match self {
            Literal(_v) => self.value() | (1 << 15),
            Jump(_v) => self.value(),
            Conditional(_v) => self.value() | (1 << 13),
            Call(_v) => self.value() | (2 << 13),
            ALU(alu) => alu.compile(),
        }
    }

    pub fn show(&self) -> String {
        match self {
            Literal(v) => format!("LIT     {:0>4x}", v),
            Jump(v) => format!("UBRANCH {:0>4x}", (v << 1)),
            Conditional(v) => format!("0BRANCH {:0>4x}", (v << 1)),
            Call(v) => format!("CALL    {:0>4x}", (v << 1)),
            ALU(alu) => alu.show(),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Jump(v) => write!(f, "Instruction::Jump(0x{:0>4x})", v),
            Instruction::Conditional(v) => write!(f, "Instruction::Conditional(0x{:0>4x})", v),
            Instruction::Call(v) => write!(f, "Instruction::Call(0x{:0>4x})", v),
            Instruction::Literal(v) => write!(f, "Instruction::Literal(0x{:0>4x})", v),
            Instruction::ALU(v) => write!(f, "Instruction::ALU({:?})", *v),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AluAttributes {
    opcode: u16,
    r2pc: bool,
    t2n: bool,
    t2r: bool,
    n2t: bool,
    r_dir: i8,
    d_dir: i8,
}

impl AluAttributes {
    pub fn value(&self) -> u16 {
        {
            let mut ret = self.opcode << 8;
            if self.r2pc { ret = ret | 1 << 12 }
            if self.t2n { ret = ret | 1 << 7 }
            if self.t2r { ret = ret | 1 << 6 }
            if self.n2t { ret = ret | 1 << 5 }
            ret = ret | ((self.r_dir & 3) as u16) << 2;
            ret = ret | ((self.d_dir & 3) as u16) << 0;
            ret
        }
    }

    pub fn compile(&self) -> u16 { self.value() | (3 << 13) }

    pub fn show(&self) -> String {
        let mut s = "ALU     ".to_string();
        s = format!("{}{}", s, OPCODE_NAMES[self.opcode as usize]);
        if self.r2pc { s = format!("{} R→PC", s) }
        if self.t2n { s = format!("{} T→N", s) }
        if self.t2r { s = format!("{} T→R", s) }
        if self.n2t { s = format!("{} N→[T]", s) }
        if self.r_dir != 0 { s = format!("{} r{:+}", s, self.r_dir) }
        if self.d_dir != 0 { s = format!("{} d{:+}", s, self.d_dir) }
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::*;

    #[test]
    fn alu_attributes() {
        println!("default = {:?}", AluAttributes::default());
        assert_eq!(
            AluAttributes::default(),
            AluAttributes { opcode: 0, r2pc: false, t2n: false, t2r: false, n2t: false, r_dir: 0, d_dir: 0 }
        );
    }

    #[test]
    fn instruction_decode() {
        let test_cases = [
            (0x0000, Jump(0x0000)),
            (0x1fff, Jump(0x1fff)),
            (0x2000, Conditional(0x0000)),
            (0x3fff, Conditional(0x1fff)),
            (0x4000, Call(0x0000)),
            (0x5fff, Call(0x1fff)),
            (0x8000, Literal(0x0000)),
            (0xffff, Literal(0x7fff)),
            (0x6000, ALU(AluAttributes { opcode: 0, r2pc: false, t2n: false, t2r: false, n2t: false, r_dir: 0, d_dir: 0 })),
            (0x6100, ALU(AluAttributes { opcode: 1, r2pc: false, t2n: false, t2r: false, n2t: false, r_dir: 0, d_dir: 0 })),
            (0x7000, ALU(AluAttributes { opcode: 0, r2pc: true, t2n: false, t2r: false, n2t: false, r_dir: 0, d_dir: 0 })),
            (0x6080, ALU(AluAttributes { opcode: 0, r2pc: false, t2n: true, t2r: false, n2t: false, r_dir: 0, d_dir: 0 })),
            (0x6040, ALU(AluAttributes { opcode: 0, r2pc: false, t2n: false, t2r: true, n2t: false, r_dir: 0, d_dir: 0 })),
            (0x6020, ALU(AluAttributes { opcode: 0, r2pc: false, t2n: false, t2r: false, n2t: true, r_dir: 0, d_dir: 0 })),
            (0x600c, ALU(AluAttributes { opcode: 0, r2pc: false, t2n: false, t2r: false, n2t: false, r_dir: -1, d_dir: 0 })),
            (0x6004, ALU(AluAttributes { opcode: 0, r2pc: false, t2n: false, t2r: false, n2t: false, r_dir: 1, d_dir: 0 })),
            (0x6003, ALU(AluAttributes { opcode: 0, r2pc: false, t2n: false, t2r: false, n2t: false, r_dir: 0, d_dir: -1 })),
            (0x6001, ALU(AluAttributes { opcode: 0, r2pc: false, t2n: false, t2r: false, n2t: false, r_dir: 0, d_dir: 1 })),
            (0x6f00, ALU(AluAttributes { opcode: 15, r2pc: false, t2n: false, t2r: false, n2t: false, r_dir: 0, d_dir: 0 })),
            (0x70e5, ALU(AluAttributes { opcode: 0, r2pc: true, t2n: true, t2r: true, n2t: true, r_dir: 1, d_dir: 1 })),
            (0x7fef, ALU(AluAttributes { opcode: 15, r2pc: true, t2n: true, t2r: true, n2t: true, r_dir: -1, d_dir: -1 })),
        ];
        for (bin, expected_instruction) in test_cases.iter() {
            let decoded = decode(*bin).unwrap();
            assert_eq!(decoded, *expected_instruction);
            // println!("decode(0x{:0>4x}) = {}", *bin, decoded)
        }
    }

    #[test]
    fn instruction_value_compile_show() {
        let test_cases = [
            (0x0000, 0, 0, "UBRANCH 0000".to_string()),
            (0x1fff, 8191, 8191, "UBRANCH 3ffe".to_string()),
            (0x2000, 0, 8192, "0BRANCH 0000".to_string()),
            (0x3fff, 8191, 16383, "0BRANCH 3ffe".to_string()),
            (0x4000, 0, 16384, "CALL    0000".to_string()),
            (0x5fff, 8191, 24575, "CALL    3ffe".to_string()),
            (0x8000, 0, 32768, "LIT     0000".to_string()),
            (0xffff, 32767, 65535, "LIT     7fff".to_string()),
            (0x6000, 0, 24576, "ALU     T".to_string()),
            (0x6100, 256, 24832, "ALU     N".to_string()),
            (0x7000, 4096, 28672, "ALU     T R→PC".to_string()),
            (0x6080, 128, 24704, "ALU     T T→N".to_string()),
            (0x6040, 64, 24640, "ALU     T T→R".to_string()),
            (0x6020, 32, 24608, "ALU     T N→[T]".to_string()),
            (0x600c, 12, 24588, "ALU     T r-1".to_string()),
            (0x6004, 4, 24580, "ALU     T r+1".to_string()),
            (0x6003, 3, 24579, "ALU     T d-1".to_string()),
            (0x6001, 1, 24577, "ALU     T d+1".to_string()),
            (0x6f00, 3840, 28416, "ALU     Nu<T".to_string()),
            (0x70e5, 4325, 28901, "ALU     T R→PC T→N T→R N→[T] r+1 d+1".to_string()),
            (0x7fef, 8175, 32751, "ALU     Nu<T R→PC T→N T→R N→[T] r-1 d-1".to_string()),
        ];
        for (bin, expected_value, expected_compile, expected_show) in test_cases.iter() {
            let decoded = decode(*bin).unwrap();

            let decoded_value = decoded.value();
            assert_eq!(decoded_value, *expected_value);
            // println!("i.value()   : {}", decoded_value);

            let decoded_compile = decoded.compile();
            assert_eq!(decoded_compile, *expected_compile);
            // println!("i.compile()   : {}", decoded_compile)

            let decoded_show = decoded.show();
            assert_eq!(decoded_show, *expected_show);
            // println!("i.show()   : {}", decoded_show);
        }
    }
}