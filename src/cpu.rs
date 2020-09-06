use crate::stack::Stack;

#[allow(dead_code)]
pub struct CPU {
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
}

