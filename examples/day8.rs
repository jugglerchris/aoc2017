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

fn solve1(input: &str) -> (isize, isize) {
    let data = input.lines()
                    .map(parse_insn)
                    .collect::<Vec<_>>();
    let mut regs: HashMap<&str, isize> = HashMap::new();
    let mut maxval = 0isize;

    use Comparison::*;

    for insn in &data {
        let condval: isize = *regs.get(insn.cond_src.as_str()).unwrap_or(&0);
        let cmpval = insn.cmpval;
        let test = match insn.cmp {
            EQ => condval == cmpval,
            NE => condval != cmpval,
            LT => condval < cmpval,
            LE => condval <= cmpval,
            GT => condval > cmpval,
            GE => condval >= cmpval,
        };
        if test {
            let &targetval = regs.get(insn.target.as_str()).unwrap_or(&0);
            let newval =  match insn.op {
                Operation::Inc => { targetval + insn.adjval },
                Operation::Dec => { targetval - insn.adjval },
            };
            regs.insert(&insn.target, newval);
            if newval > maxval {
                maxval = newval;
            }
        }
    }
    (*regs.iter().map(|(_,v)| v).max().unwrap(), maxval)
}

fn main() {
    let input = aoc2017::get_input(8).unwrap();

    assert_eq!(solve1(r#"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"#), (1, 10));

    println!("Answer 1, 2: {:?}", solve1(&input));
}
