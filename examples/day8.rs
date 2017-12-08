#![feature(conservative_impl_trait)]
#[macro_use] extern crate aoc2017;
#[macro_use] extern crate lazy_static;

use std::collections::*;
use std::str::FromStr;

#[derive(Debug,Clone,Eq,PartialEq)]
enum Operation {
    Inc,
    Dec,
}

#[derive(Debug,Clone,Eq,PartialEq)]
enum Comparison {
    EQ,
    NE,
    LT,
    GT,
    LE,
    GE,
}

#[derive(Debug,Clone,Eq,PartialEq)]
struct Instruction {
    target: String,
    op: Operation,
    adjval: isize,
    cond_src: String,
    cmp: Comparison,
    cmpval: isize,
}

fn parse_cmp(s: &str) -> Comparison {
    use Comparison::*;
    match s {
        "==" => EQ,
        "!=" => NE,
        "<" => LT,
        ">" => GT,
        "<=" => LE,
        ">=" => GE,
        _ => panic!(),
    }
}

impl FromStr for Comparison {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_cmp(s))
    }
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Operation::*;
        match s {
            "inc" => Ok(Inc),
            "dec" => Ok(Dec),
            _ => panic!(),
        }
    }
}

regex_parser!(parse_insn: Instruction {
    INSN = r#"^(\w+) (inc|dec) (-?\d+) if (\w+) (==|!=|<|>|<=|>=) (-?\d+)$"# =>
        | target: String, op: Operation, adjval: isize, cond_src: String,
          cmp: Comparison, cmpval: isize |
            Instruction {
                target: target,
                op: op,
                adjval: adjval,
                cond_src: cond_src,
                cmp: cmp,
                cmpval: cmpval
            }
});

#[test]
fn test_parse() {
    assert_eq!(parse_insn("b inc 5 if a > 1"),
               Instruction {
                   target: "b".into(),
                   op: Operation::Inc,
                   adjval: 5,
                   cond_src: "a".into(),
                   cmp: Comparison::GT,
                   cmpval: 1
               });
}

fn main () {
}