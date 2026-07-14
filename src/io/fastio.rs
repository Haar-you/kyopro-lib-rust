//! 高速な標準入出力
#![allow(clippy::new_without_default)]

use std::fmt::Display;
use std::io::{Read, Write};

/// 高速な標準入出力
pub struct FastIO {
    in_bytes: Vec<u8>,
    in_cur: usize,
    out_buf: std::io::BufWriter<std::io::Stdout>,
}

impl FastIO {
    /// [`FastIO`]を生成する。
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

    /// 1バイトだけ読み出す。
    #[inline]
    pub fn getc(&mut self) -> Option<u8> {
        let c = *self.in_bytes.get(self.in_cur)?;
        self.in_cur += 1;
        Some(c)
    }

    /// 1バイトだけ先読みする。
    #[inline]
    pub fn peek(&self) -> Option<u8> {
        Some(*self.in_bytes.get(self.in_cur)?)
    }

    /// `is_ascii_whitespace`が`true`である間を読み飛ばす。
    #[inline]
    pub fn skip(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_whitespace()) {
            self.in_cur += 1;
        }
    }

    /// [`u64`]型の数値を読み出す。
    pub fn read_u64(&mut self) -> u64 {
        self.skip();
        let mut ret: u64 = 0;

        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            ret = ret * 10 + (self.in_bytes[self.in_cur] - b'0') as u64;
            self.in_cur += 1;
        }

        ret
    }

    /// [`u32`]型の数値を読み出す。
    pub fn read_u32(&mut self) -> u32 {
        self.read_u64() as u32
    }

    /// [`usize`]型の数値を読み出す。
    pub fn read_usize(&mut self) -> usize {
        self.read_u64() as usize
    }

    /// [`i64`]型の数値を読み出す。
    pub fn read_i64(&mut self) -> i64 {
        self.skip();
        let mut ret: i64 = 0;

        let minus = if self.peek() == Some(b'-') {
            self.in_cur += 1;
            true
        } else {
            false
        };

        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            ret = ret * 10 + (self.in_bytes[self.in_cur] - b'0') as i64;
            self.in_cur += 1;
        }

        if minus {
            ret = -ret;
        }

        ret
    }

    /// [`i32`]型の数値を読み出す。
    pub fn read_i32(&mut self) -> i32 {
        self.read_i64() as i32
    }

    /// [`isize`]型の数値を読み出す。
    pub fn read_isize(&mut self) -> isize {
        self.read_i64() as isize
    }

    /// [`f64`]型の数値を読み出す。
    pub fn read_f64(&mut self) -> f64 {
        self.read_chars()
            .into_iter()
            .collect::<String>()
            .parse()
            .unwrap()
    }

    /// 文字列を[`Vec<char>`]として読み出す。
    pub fn read_chars(&mut self) -> Vec<char> {
        self.read_bytes().into_iter().map(Into::into).collect()
    }

    /// 文字列を[`String`]として読み出す。
    pub fn read_string(&mut self) -> String {
        self.read_chars().into_iter().collect()
    }

    /// 文字列を[`Vec<u8>`]として読み出す。
    pub fn read_bytes(&mut self) -> Vec<u8> {
        self.skip();
        let mut ret = vec![];

        while self.peek().is_some_and(|c| c.is_ascii_graphic()) {
            ret.push(self.in_bytes[self.in_cur]);
            self.in_cur += 1;
        }

        ret
    }

    /// `is_ascii_whitespace`でない文字を一文字だけ読み出す。
    pub fn read_char(&mut self) -> char {
        self.skip();
        self.getc().unwrap().into()
    }

    /// `u32`型の数値を標準出力に書き込む。
    pub fn writeln_u32(&mut self, mut n: u32) {
        if n == 0 {
            self.out_buf.write_all(b"0\n").unwrap();
            return;
        }

        let mut buf = [b' '; 10];
        let mut i = 9;
        while n > 0 {
            buf[i] = (n % 10) as u8 + b'0';
            n /= 10;
            i -= 1;
        }

        self.out_buf.write_all(&buf).unwrap();
        self.out_buf.write_all(b"\n").unwrap();
    }

    /// `s`を標準出力に書き込む。
    pub fn write<T: Display>(&mut self, s: T) {
        self.out_buf.write_all(s.to_string().as_bytes()).unwrap();
    }

    /// `s`を標準出力に**逆順**に書き込む。
    pub fn write_rev<T: Display>(&mut self, s: T) {
        let mut s = s.to_string().as_bytes().to_vec();
        s.reverse();
        self.out_buf.write_all(&s).unwrap();
    }

    /// `s`と改行文字を標準出力に書き込む。
    pub fn writeln<T: Display>(&mut self, s: T) {
        self.write(s);
        self.out_buf.write_all(b"\n").unwrap();
    }

    /// `s`の**逆順**と改行文字を標準出力に書き込む。
    pub fn writeln_rev<T: Display>(&mut self, s: T) {
        self.write_rev(s);
        self.out_buf.write_all(b"\n").unwrap();
    }

    /// `u8`のスライスを標準出力に書き込む。
    pub fn write_bytes(&mut self, s: &[u8]) {
        self.out_buf.write_all(s).unwrap();
    }

    /// イテレータ`it`の中身を空白区切りで標準出力に書き込む。
    pub fn writeln_intersperse<I>(&mut self, it: I)
    where
        I: IntoIterator,
        I::Item: Display,
    {
        let mut it = it.into_iter();
        if let Some(s) = it.next() {
            self.write(s);
        }

        for s in it {
            self.write_bytes(b" ");
            self.write(s);
        }

        self.write_bytes(b"\n");
    }
}

impl Drop for FastIO {
    fn drop(&mut self) {
        self.out_buf.flush().unwrap();
    }
}
