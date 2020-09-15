use std::io::{Write, Cursor, Read};
use std::iter::FromIterator;

/// Console
///
/// IO console for J1 cpu
///
#[allow(dead_code)]
#[derive(Clone)]
pub struct Console {
    pub ar1: [u8; 1],
    pub reader: Cursor<Vec<u8>>,
    pub writer: Vec<u8>,
    pub log: Vec<char>,
}

impl Console {
    /// Constructs a new, empty `Console`.
    /// # Examples
    ///
    /// ```
    /// let mut console = Console::new();
    /// ```
    pub fn new() -> Self {
        Console {
            ar1: [0u8],
            reader: Cursor::new(Vec::new()),
            writer: Vec::new(),
            log: Vec::new(),
        }
    }

    pub fn write_char(&mut self, v: u8) {
        if v != b'\r' {
            self.log.push(v as char);
            let _ = write!(&mut self.writer, "{}", v as char);
        }
    }

    pub fn read_char(&mut self) -> char {
        let _r = self.reader.read_exact(&mut self.ar1);
        self.ar1[0] as char
    }

    /// Loads the console read buffer with some bytes
    ///
    /// # Examples
    ///
    /// ```
    /// let mut console = Console::new();
    /// let xs = b"1 2 + .\n".to_vec();
    /// console.load(&mut xs.clone());
    /// let ys = console.reader.get_ref();
    /// assert_eq!(xs.len(), ys.len());
    /// assert_eq!(xs, *ys);
    ///
    /// ```
    pub fn load(&mut self, xs: &mut Vec<u8>) {
        let buf = self.reader.get_mut();
        buf.clear();
        buf.append(xs);
        self.reader.set_position(0)
    }

    pub fn get_log(&self) -> String {
        String::from_iter(self.log.clone())
    }
    pub fn get_writer(&self) -> String { String::from_utf8(self.writer.clone()).unwrap() }
}


#[cfg(test)]
mod tests {
    use crate::console::*;

    #[test]
    fn read_and_write_char() {
        let xs = b"1 2 + .\n".to_vec();
        let mut console = Console {
            ar1: [0u8],
            reader: Cursor::new(xs.clone()),
            writer: Vec::new(),
            log: Vec::new(),
        };

        for x in xs.iter() {
            console.write_char(*x);
        }

        assert_eq!(console.reader.get_ref().len(), xs.len());
        assert_eq!(std::str::from_utf8(&xs[..]).unwrap().to_string(), console.get_log());
        println!("\nlog = {}", console.get_log());
    }

    #[test]
    fn load() {
        let mut console = Console::new();
        let xs = b"1 2 + .\n".to_vec();
        console.load(&mut xs.clone());
        let ys = console.reader.get_ref();
        assert_eq!(xs.len(), ys.len());
        assert_eq!(xs, *ys);
    }
}

