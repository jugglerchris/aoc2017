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
                static ref $re_name: $crate::Regex = $crate::Regex::new($re).unwrap();
                 )*
            }
            fn $fname(s: &str) -> $typ {
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

pub fn get_input(n: u32) -> io::Result<String> {
    let filename = format!("data/{}.txt", n);
    let mut f = try!(File::open(&filename));
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    Ok(data)
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
