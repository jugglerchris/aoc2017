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
