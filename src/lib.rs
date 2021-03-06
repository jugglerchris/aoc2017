extern crate regex;
#[macro_use] extern crate lazy_static;

use std::fs::File;
use std::io;
use std::io::{Read};
use std::str::FromStr;
use std::fmt::Debug;

pub use regex::Regex;

#[macro_export]
macro_rules! regex_parser {
    ($fname:ident : $typ:ty { $($re_name:ident = $re:expr => |$($cap:ident : $capty:ty),*| $res:expr ),* }) =>
        {
            lazy_static! {
                $(
                pub static ref $re_name: $crate::Regex = $crate::Regex::new($re).unwrap();
                 )*
            }
            pub fn $fname(s: &str) -> $typ {
                $(
                    if let Some(cap) = $re_name.captures(s) {
                        return {
                            let mut capno = 0;
                            $(capno += 1; let $cap: $capty = cap[capno].parse().unwrap(); )*
                            $res
                        };
                    }
                )*
                panic!("Failed to parse: [[{}]]", s)
            }
        }
}

pub fn get_input_str(s: &str) -> io::Result<String> {
    let filename = format!("data/{}.txt", s);
    let mut f = try!(File::open(&filename));
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    Ok(data)
}

pub fn get_input(n: u32) -> io::Result<String> {
    get_input_str(&format!("{}", n))
}

pub fn parse_rows<T:FromStr+Debug>(data: &str) -> Vec<Vec<T>>
   where <T as FromStr>::Err: Debug
{
    data.lines()
        .map(|line| line.split_whitespace()
                        .map(|s| s.parse().expect("Failed to parse"))
                        .collect())
        .collect()
}

pub fn parse_rows_str(data: &str) -> Vec<Vec<&str>>
{
    data.lines()
        .map(|line| line.split_whitespace()
                        .collect())
        .collect()
}

pub fn parse_lines<T:FromStr+Debug>(data: &str) -> Vec<T>
   where <T as FromStr>::Err: Debug
{
    data.lines()
        .map(|s| s.parse().expect("Failed to parse"))
        .collect()
}

pub struct Hasher {
    lengths: Vec<usize>,
    cur_positions: Vec<usize>,
    skip: usize,
    cur_pos: usize,
}

impl Hasher {
    // Make a hasher from a string
    pub fn new_from_string(input: &str) -> Hasher {
        let lengths: Vec<usize> =
            input.as_bytes()
                 .iter()
                 .chain(&[17, 31, 73, 47, 23])
                 .map(|&x| x as usize)
                 .collect();
        Hasher::new(&lengths)
    }

    // Create a hasher from raw lengths
    pub fn new(lengths: &[usize]) -> Hasher {
        Hasher {
            lengths: lengths.into(),
            cur_positions: (0..256).collect(),
            skip: 0,
            cur_pos: 0,
        }
    }

    pub fn one_round(&mut self) {
        for &l in &self.lengths {
            let knotstart = self.cur_pos;
            let knotend = knotstart + l - 1;
            for i in 0..l/2 {
                self.cur_positions.swap((knotstart+i)&0xff, (knotend - i)&0xff);
            }
            self.cur_pos += l + self.skip;
            self.skip += 1;
        }
    }

    pub fn as_hex(&self) -> String {
        let mut result = String::new();
        for i in 0..16 {
            let mut b = 0;
            for j in 0..16 {
                b ^= self.cur_positions[i*16 + j];
            }
            result.push_str(&format!("{:02x}", b));
        }
        result
    }
    pub fn bits_set(&self) -> usize {
        let mut result = 0;
        for i in 0..16 {
            let mut b = 0;
            for j in 0..16 {
                b ^= self.cur_positions[i*16 + j];
            }
            result += b.count_ones() as usize;
        }
        result
    }
    pub fn into_bool_vec(self) -> Vec<bool> {
        let mut result = Vec::new();
        for i in 0..16 {
            let mut b = 0;
            for j in 0..16 {
                b ^= self.cur_positions[i*16 + j];
            }
            for _ in 0..8 {
                result.push((b & 0x80) != 0);
                b = b << 1;
            }
        }
        result
    }
}

pub mod cpu {
    use std::str::FromStr;

    #[derive(Debug,Clone,Eq,PartialEq,Copy)]
    pub enum Operand {
        Reg(usize),
        Val(isize),
    }
    pub use self::Operand::*;

    regex_parser!(parse_operand: Operand {
        VAL = r#"^(-?\d+)$"# => | val: isize | Operand::Val(val),
        REG = r#"^(\w)$"# => | reg: String | Operand::Reg((reg.chars().next().unwrap() as u8 - b'a') as usize)
    });

    impl FromStr for Operand {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(parse_operand(s))
        }
    }


    #[derive(Debug,Clone,Eq,PartialEq)]
    pub enum Insn {
        Snd(Operand),
        Set(Operand, Operand),
        Add(Operand, Operand),
        Sub(Operand, Operand),
        Mul(Operand, Operand),
        Mod(Operand, Operand),
        Rcv(Operand),
        Jgz(Operand, Operand),
        Jnz(Operand, Operand),
    }
    pub use self::Insn::*;

    regex_parser!(parse_insn: Insn {
        SND = r#"snd (.*)$"# => | op: Operand | Snd(op),
        SET = r#"set (.*) (.*)$"# => | op1: Operand, op2: Operand | Set(op1, op2),
        ADD = r#"add (.*) (.*)$"# => | op1: Operand, op2: Operand | Add(op1, op2),
        SUB = r#"sub (.*) (.*)$"# => | op1: Operand, op2: Operand | Sub(op1, op2),
        MUL = r#"mul (.*) (.*)$"# => | op1: Operand, op2: Operand | Mul(op1, op2),
        MOD = r#"mod (.*) (.*)$"# => | op1: Operand, op2: Operand | Mod(op1, op2),
        RCV = r#"rcv (.*)$"# => | op: Operand | Rcv(op),
        JGZ = r#"jgz (.*) (.*)$"# => | op1: Operand, op2: Operand | Jgz(op1, op2),
        JNZ = r#"jnz (.*) (.*)$"# => | op1: Operand, op2: Operand | Jnz(op1, op2)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn parse_rows_test()
    {
        assert_eq!(vec![vec![1,2,3],
                        vec![4,5]],
                   parse_rows("1 2 3\n4 5"));
    }
}
