use std::io::{BufRead, Write};
use std::iter::FromIterator;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Console<R, W> {
    pub reader: R,
    pub writer: W,
    pub log: Vec<char>,
}

impl<R, W> Console<R, W>
    where
        R: BufRead,
        W: Write,
{
    pub fn write_char(&mut self, v: u8) {
        self.log.push(v as char);
        let _ = write!(&mut self.writer, "{}", v as char);
    }

    pub fn read_char(&mut self) -> char {
        let mut ar1 = [0u8];
        let _r = self.reader.read_exact(&mut ar1);
        let ch = ar1[0] as char;
        ch
    }

    pub fn get_log(&self) -> String {
        String::from_iter(self.log.clone())
    }
}


#[cfg(test)]
mod tests {
    use crate::console::*;

    #[test]
    fn console() {
        let xs = b"1 2 + .\n";
        let mut console = Console {
            reader: &xs[..],
            writer: Vec::new(),
            log: Vec::new(),
        };

        for x in xs.iter() {
            console.write_char(*x);
        }

        assert_eq!(console.reader.len(), xs.len());
        assert_eq!(std::str::from_utf8(xs).unwrap().to_string(), console.get_log());
        println!("\nlog = {}", console.get_log());
    }
}

