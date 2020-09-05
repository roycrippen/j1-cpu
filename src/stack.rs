#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Stack {
    data: [u16; 32],
    sp: i8,
}

#[allow(dead_code)]
impl Stack {
    fn move_sp(&mut self, dir: i8) {
        self.sp = (self.sp + dir) & 0x1f
    }

    fn push(&mut self, v: u16) {
        self.sp = (self.sp + 1) & 0x1f;
        self.data[self.sp as usize] = v
    }

    fn pop(&mut self) -> u16 {
        let sp = self.sp;
        self.sp = (self.sp - 1) & 0x1f;
        self.data[sp as usize]
    }

    fn peek(&self) -> u16 {
        self.data[self.sp as usize]
    }

    fn replace(&mut self, v: u16) {
        self.data[self.sp as usize] = v
    }

    fn depth(&self) -> i8 {
        self.sp
    }

    fn dump(&self) -> Vec<u16> {
        let last = (self.sp + 1) as usize;
        self.data[1..last].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::*;

    #[test]
    fn stack_basic() {
        let mut s = Stack::default();
        assert_eq!(s.sp, 0);
        assert_eq!(s.dump().len(), 0);

        s.push(1);
        s.push(2);
        s.push(3);

        assert_eq!(s.dump().len(), 3);
        assert_eq!(s.peek(), 3);
        assert_eq!(s.dump().len(), 3);

        s.replace(4);
        assert_eq!(s.pop(), 4);
        assert_eq!(s.dump().len(), 2);
        assert_eq!(s.dump().len(), s.depth() as usize);

        s.push(3);
        s.push(4);
        s.push(5);
        s.push(6);
        s.push(7);
        s.push(8);
        s.push(9);

        println!("{:?}", s.data);
        // println!("{:?}", s.dump());
        // println!("s.sp = {}", s.sp);
        assert_eq!(s.dump().len(), s.depth() as usize);
        assert_eq!(s.depth(), 9);
        assert_eq!(s.sp, 9);
    }

    #[test]
    fn stack_wrap() {
        let mut s = Stack::default();
        for i in 1..34 {
            s.push(i)
        }
        // println!("{:?}", s.dump());
        // println!("{:?}", s.data);
        assert_eq!(s.dump(), [33]);
        assert_eq!(s.depth(), 1);
        let data = [
            32, 33, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ];
        assert_eq!(s.data, data)
    }

    #[test]
    fn stack_move_pointer() {
        let mut s = Stack::default();

        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.depth(), 3);
        assert_eq!(s.peek(), 3);

        s.move_sp(-1);
        assert_eq!(s.depth(), 2);
        assert_eq!(s.peek(), 2);
        // println!("{:?}", s.dump());
        // println!("{:?}", s.data);

        s.move_sp(1);
        assert_eq!(s.depth(), 3);
        assert_eq!(s.peek(), 3);
        // println!("{:?}", s.dump());
        // println!("{:?}", s.data);
    }
}
