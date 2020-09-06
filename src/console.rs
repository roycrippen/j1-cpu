use std::io::{Cursor, Read, Write};
use std::str;

#[allow(dead_code)]
pub struct Console<T: Read + Write> {
    ar1: [u8; 1],
    in_buf: T,
    out_buf: T,
    pub should_log: bool,
    log: Vec<String>,
}

#[allow(dead_code)]
impl <T: Read + Write> Console<T> {
    fn get_log(&self) -> Vec<String> {
        self.log.clone()
    }

    fn clear_log(&mut self) {
        self.log.clear();
    }

}

pub trait IO {
    fn new(should_log: bool) -> Self;
    fn load_in_buf(&mut self, xs: &mut [u8]);
    fn in_buff_len(&self) -> usize;
    fn read_byte(&mut self) -> Option<u8>;
    fn read_all_bytes(&mut self) -> Vec<u8>;
    fn write_byte(&mut self, v: u8);
}


pub type MockConsole = Cursor<Vec<u8>>;

// testing io buffers
impl IO for Console<MockConsole> {
    fn new(should_log: bool) -> Self {
        Console {
            ar1: [0],
            in_buf: Cursor::new(Vec::new()),
            out_buf: Cursor::new(Vec::new()),
            should_log,
            log: Vec::new(),
        }
    }

    fn load_in_buf(&mut self, xs: &mut [u8]) {
        self.in_buf.write_all(xs).unwrap_or(());
        self.in_buf.set_position(0);
        if self.should_log {
            let s = str::from_utf8(xs).unwrap_or("").to_string();
            let mut ss = s.lines().map(Into::into).collect();
            self.log.append(&mut ss);
        }
    }

    fn in_buff_len(&self) -> usize {
        self.in_buf.get_ref().len()
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.in_buf.position() == self.in_buf.get_ref().len() as u64 {
            None
        } else {
            let _ = self.in_buf.read_exact(&mut self.ar1);
            Some(self.ar1[0])
        }
    }

    fn read_all_bytes(&mut self) -> Vec<u8> {
        self.in_buf.set_position(0);
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

    #[test]
    fn mock_read_bytes() {
        let console = &mut Console::<MockConsole>::new(true);

        let mut cmds: Vec<u8> = "1 2 + .s\n".bytes().collect();
        console.load_in_buf(&mut cmds);
        assert_eq!(cmds.len(), console.in_buff_len());
        assert_eq!(0, console.in_buf.position());
        // println!("buff len = {:?}, buff pos = {}", console.io.get_ref().len(), console.io.position());

        let xs = console.read_all_bytes();
        assert_eq!(cmds, xs);
        assert_eq!(None, console.read_byte());
        assert_eq!(console.in_buff_len() as u64, console.in_buf.position());
    }

    #[test]
    fn log() {
        let console = &mut Console::<MockConsole>::new(true);

        let mut cmds: Vec<u8> = "1 2 + .s\n3 * .s\n".bytes().collect();
        console.load_in_buf(&mut cmds);

        cmds = "dup\n".bytes().collect();
        console.load_in_buf(&mut cmds);

        assert_eq!(vec!["1 2 + .s", "3 * .s", "dup"], console.get_log());
        // println!("{:?}", console.get_log())

        console.clear_log();
        assert_eq!(0, console.get_log().len());
    }
}

