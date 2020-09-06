use std::io::{Read, Cursor};
use std::io::Write;

#[allow(dead_code)]
pub struct Console<T: Read + Write> {
    ar1: [u8; 1],
    in_buff: T,
    out_buff: T,
}

trait IO {
    fn read(&mut self, xs: &mut [u8]);
    fn buff_len(&self) -> usize;
    fn read_byte(&mut self) -> Option<u8>;
    fn read_all_bytes(&mut self) -> Vec<u8>;
    fn write_byte(&mut self, v: u8);
}

// testing io buffers
impl IO for Console<&mut Cursor<Vec<u8>>> {
    fn read(&mut self, xs: &mut [u8]) {
        self.in_buff.write_all(xs).unwrap_or(());
        self.in_buff.set_position(0);
    }

    fn buff_len(&self) -> usize {
        self.in_buff.get_ref().len()
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.in_buff.position() == self.in_buff.get_ref().len() as u64 {
            None
        } else {
            let _ = self.in_buff.read_exact(&mut self.ar1);
            Some(self.ar1[0])
        }
    }

    fn read_all_bytes(&mut self) -> Vec<u8> {
        self.in_buff.set_position(0);
        let mut xs: Vec<u8> = Vec::new();
        while let Some(v) = self.read_byte() {
            xs.push(v);
        }
        xs
    }

    fn write_byte(&mut self, _v: u8) {}
}

#[cfg(test)]
mod tests {
    use crate::console::*;
    use std::io::{Cursor};

    #[test]
    fn mock_read_bytes() {
        let console = &mut Console { ar1: [0], in_buff: &mut Cursor::new(Vec::new()), out_buff: &mut Cursor::new(Vec::new()) };

        let mut cmds: Vec<u8> = "1 2 + .s\n".bytes().collect();
        console.read(&mut cmds);
        assert_eq!(cmds.len(), console.in_buff.get_ref().len());
        assert_eq!(0, console.in_buff.position());
        // println!("buff len = {:?}, buff pos = {}", console.io.get_ref().len(), console.io.position());

        let xs = console.read_all_bytes();
        assert_eq!(cmds, xs);
        assert_eq!(None, console.read_byte());
        assert_eq!(console.buff_len() as u64, console.in_buff.position());
    }
}

