#![allow(clippy::new_without_default)]

use std::io::{Read, Write};

pub struct FastIO {
    in_bytes: Vec<u8>,
    in_cur: usize,
    out_buf: std::io::BufWriter<std::io::Stdout>,
}

impl FastIO {
    pub fn new() -> Self {
        let mut s = vec![];
        std::io::stdin().read_to_end(&mut s).unwrap();

        let cout = std::io::stdout();

        Self {
            in_bytes: s,
            in_cur: 0,
            out_buf: std::io::BufWriter::new(cout),
        }
    }

    #[inline]
    pub fn getc(&mut self) -> Option<u8> {
        if self.in_cur < self.in_bytes.len() {
            self.in_cur += 1;
            Some(self.in_bytes[self.in_cur])
        } else {
            None
        }
    }

    #[inline]
    pub fn peek(&self) -> Option<u8> {
        if self.in_cur < self.in_bytes.len() {
            Some(self.in_bytes[self.in_cur])
        } else {
            None
        }
    }

    #[inline]
    pub fn skip(&mut self) {
        while self.peek().map_or(false, |c| c.is_ascii_whitespace()) {
            self.in_cur += 1;
        }
    }

    pub fn read_u64(&mut self) -> u64 {
        self.skip();
        let mut ret: u64 = 0;

        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            ret = ret * 10 + (self.in_bytes[self.in_cur] - b'0') as u64;

            self.in_cur += 1;
        }

        ret
    }

    pub fn read_i64(&mut self) -> i64 {
        self.skip();
        let mut ret: i64 = 0;

        let minus = if self.peek() == Some(b'-') {
            self.in_cur += 1;
            true
        } else {
            false
        };

        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            ret = ret * 10 + (self.in_bytes[self.in_cur] - b'0') as i64;
            self.in_cur += 1;
        }

        if minus {
            ret = -ret;
        }

        ret
    }

    pub fn read_chars(&mut self) -> Vec<char> {
        self.skip();
        let mut ret = vec![];

        while self.peek().map_or(false, |c| c.is_ascii_graphic()) {
            ret.push(self.in_bytes[self.in_cur] as char);
            self.in_cur += 1;
        }

        ret
    }

    pub fn write(&mut self, s: &str) {
        self.out_buf.write_all(s.as_bytes()).unwrap();
    }

    pub fn writeln(&mut self, s: &str) {
        self.write(s);
        self.out_buf.write_all(&[b'\n']).unwrap();
    }
}

impl Drop for FastIO {
    fn drop(&mut self) {
        self.out_buf.flush().unwrap();
    }
}
