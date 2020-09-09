use std::io::{Cursor, Read, Write};
use std::str;
use std::process::Stdio;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Console<T: IO> {
    pub buf: T,
    pub should_log: bool,
    pub log: Vec<u8>,
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

    pub fn load(&mut self, xs: &mut [u8]) {
        self.buf.load_buf(xs);
    }

    pub fn read(&mut self) -> Option<u8> {
        self.buf.read_byte()
    }

    pub fn write(&mut self, v: u8) {
        self.buf.write_byte(v);
        if self.should_log {
            self.log.push(v);
        }
    }

    pub fn get_log(&self) -> Vec<String> {
        let s = str::from_utf8(&*self.log).unwrap_or("").to_string();
        s.lines().map(Into::into).collect()
    }

    pub fn clear_log(&mut self) {
        self.log.clear();
    }
}

pub trait IO {
    fn new() -> Self;
    fn load_buf(&mut self, xs: &mut [u8]);
    fn buf_has_char(&self) -> u16;
    fn read_byte(&mut self) -> Option<u8>;
    fn write_byte(&mut self, v: u8);
}

impl IO for Stdio {
    fn new() -> Self {
        unimplemented!()
    }

    fn load_buf(&mut self, _xs: &mut [u8]) {
        unimplemented!()
    }

    fn buf_has_char(&self) -> u16 {
        unimplemented!()
    }

    fn read_byte(&mut self) -> Option<u8> {
        unimplemented!()
    }

    fn write_byte(&mut self, _v: u8) {
        unimplemented!()
    }
}

pub type MockConsole = Cursor<Vec<u8>>;

impl IO for MockConsole {
    fn new() -> Self { Cursor::new(Vec::new()) }

    fn load_buf(&mut self, xs: &mut [u8]) {
        self.write_all(xs).unwrap_or(());
        self.set_position(0);
    }

    fn buf_has_char(&self) -> u16 {
        if self.get_ref().len() as u64 - self.position() > 0 { 1 } else { 0 }
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.get_ref().len() == 0 {
            return None;
        }
        let mut ar1 = [0u8];
        let _ = self.read_exact(&mut ar1);
        if self.position() == self.get_ref().len() as u64 {
            self.get_mut().clear();
            self.set_position(0);
        }
        Some(ar1[0])
    }

    fn write_byte(&mut self, _v: u8) {
        // println!("{}", _v as char)
    }
}

#[cfg(test)]
mod tests {
    use crate::console::*;

    #[test]
    fn mock_read_bytes() {
        let console = &mut Console::<MockConsole>::new(true);

        let mut cmds: Vec<u8> = "1 2 + .s\n".bytes().collect();
        console.load(&mut cmds);
        assert_eq!(cmds.len(), console.buf.get_ref().len());
        assert_eq!(0, console.buf.position());
        // println!("buff len = {:?}, buff pos = {}", console.io.get_ref().len(), console.io.position());
    }

    #[test]
    fn log() {
        let console = &mut Console::<MockConsole>::new(true);

        let mut cmds: Vec<u8> = "1 2 + .s\n3 * .s\n".bytes().collect();
        console.load(&mut cmds);

        assert_eq!(16, console.buf.get_ref().len());


        cmds.iter().for_each(|b: &u8| { console.write(*b) });
        println!("{:?}", console.get_log());
        assert_eq!(vec!["1 2 + .s", "3 * .s"], console.get_log());

        console.clear_log();
        assert_eq!(0, console.get_log().len());
    }
}

