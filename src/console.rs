use std::io::{Cursor, Read, Write};
use std::str;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Console<T: IO> {
    pub buf: T,
    pub should_log: bool,
    pub log: Vec<String>,
}

#[allow(dead_code)]
impl<T: IO> Console<T> {
    pub fn new(should_log: bool) -> Self {
        Console {
            buf: T::new(),
            should_log,
            log: Vec::new(),
        }
    }

    pub fn read(&mut self, xs: &mut [u8]) {
        self.buf.load_buf(xs);
        if self.should_log {
            let s = str::from_utf8(xs).unwrap_or("").to_string();
            let mut ss = s.lines().map(Into::into).collect();
            self.log.append(&mut ss);
        }
    }

    pub fn get_log(&self) -> Vec<String> {
        self.log.clone()
    }

    pub fn clear_log(&mut self) {
        self.log.clear();
    }
}

pub trait IO {
    fn new() -> Self;
    fn load_buf(&mut self, xs: &mut [u8]);
    fn buf_len(&self) -> usize;
    fn read_byte(&mut self) -> Option<u8>;
    fn read_all_bytes(&mut self) -> Vec<u8>;
    fn write_byte(&mut self, v: u8);
}


pub type MockConsole = Cursor<Vec<u8>>;

impl IO for MockConsole {
    fn new() -> Self { Cursor::new(Vec::new()) }

    fn load_buf(&mut self, xs: &mut [u8]) {
        self.write_all(xs).unwrap_or(());
        self.set_position(0);
    }

    fn buf_len(&self) -> usize {
        self.get_ref().len() - self.position() as usize
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.position() == self.get_ref().len() as u64 {
            None
        } else {
            let mut ar1 = [0u8];
            let _ = self.read_exact(&mut ar1);
            Some(ar1[0])
        }
    }

    fn read_all_bytes(&mut self) -> Vec<u8> {
        self.set_position(0);
        let mut xs: Vec<u8> = Vec::new();
        while let Some(v) = self.read_byte() {
            xs.push(v);
        }
        xs
    }

    fn write_byte(&mut self, _v: u8) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::console::*;

    #[test]
    fn mock_read_bytes() {
        let console = &mut Console::<MockConsole>::new(true);

        let mut cmds: Vec<u8> = "1 2 + .s\n".bytes().collect();
        console.buf.load_buf(&mut cmds);
        assert_eq!(cmds.len(), console.buf.buf_len());
        assert_eq!(0, console.buf.position());
        // println!("buff len = {:?}, buff pos = {}", console.io.get_ref().len(), console.io.position());

        let xs = console.buf.read_all_bytes();
        assert_eq!(cmds, xs);
        assert_eq!(None, console.buf.read_byte());
        assert_eq!(console.buf.buf_len(), 0);
    }

    #[test]
    fn log() {
        let console = &mut Console::<MockConsole>::new(true);

        let mut cmds: Vec<u8> = "1 2 + .s\n3 * .s\n".bytes().collect();
        console.read(&mut cmds);

        cmds = "dup\n".bytes().collect();
        console.read(&mut cmds);

        assert_eq!(vec!["1 2 + .s", "3 * .s", "dup"], console.get_log());
        // println!("{:?}", console.get_log());

        console.clear_log();
        assert_eq!(0, console.get_log().len());
    }
}

