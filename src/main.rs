use core::fmt::{Display, Formatter};
use std::io::stdin;

struct Num {
    data: Vec<u8>,
    base: u8,
}

impl Num {
    pub fn from_digits(data: Vec<u8>, base: u8) -> Self {
        assert!(!data.iter().any(|d| (d + 1) > base));
        Self { data, base }
    }

    pub fn from_ascii(ascii: &[u8], base: u8) -> Self {
        let digits: Vec<u8> = ascii
            .iter()
            .map(|b| match b {
                b'0'..=b'9' => b - b'0',
                b'a'..=b'z' => b - b'a' + 10,
                _ => unimplemented!(),
            })
            .collect();
        Self::from_digits(digits, base)
    }

    pub fn from_stdin() -> Self {
        println!("Number to convert: ");
        let line = read_line();
        let ascii = line.trim().as_bytes();
        println!("Base from: ");
        let base = read_line();
        let base: u8 = base.trim().parse().unwrap();
        Self::from_ascii(&ascii, base)
    }

    pub fn from_base10(num: u32, base: u8) -> Self {
        let mut num = num.clone();
        assert!(base > 1);
        let base_u32 = base as u32;
        let mut out = Vec::new();
        while num != 0 {
            out.push((num % base_u32) as u8);
            num /= base_u32;
        }
        out.reverse();
        Self::from_digits(out, base)
    }

    pub fn to_base10(&self) -> u32 {
        assert!(self.base > 1);
        let base_u32 = self.base as u32;
        *self.data.iter().last().unwrap() as u32
            + self
                .data
                .iter()
                .rev()
                .skip(1)
                .scan(1, |exp, d| {
                    *exp *= base_u32;
                    Some(*exp * *d as u32)
                })
                .sum::<u32>()
    }

    pub fn to_base(&self, base: u8) -> Self {
        Self::from_base10(self.to_base10(), base)
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for digit in self.data.iter() {
            match digit {
                0..=9 => write!(f, "{}", char::from(b'0' + digit))?,
                10..=35 => write!(f, "{}", char::from(b'a' + (digit - 10)))?,
                _ => unimplemented!(),
            }
        }
        Ok(())
    }
}

fn read_line() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf
}

fn main() {
    let num = Num::from_stdin();
    println!("Base to: ");
    let base_to = read_line().trim().parse().unwrap();
    println!("Result: {}", &num.to_base(base_to));
}
