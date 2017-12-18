#![feature(conservative_impl_trait)]
#[macro_use] extern crate aoc2017;
#[macro_use] extern crate lazy_static;

use std::str::FromStr;

#[derive(Debug,Clone,Eq,PartialEq,Copy)]
enum Operand {
    Reg(usize),
    Val(isize),
}
use Operand::*;

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
enum Insn {
    Snd(Operand),
    Set(Operand, Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Mod(Operand, Operand),
    Rcv(Operand),
    Jgz(Operand, Operand),

}
use Insn::*;

regex_parser!(parse_insn: Insn {
    SND = r#"snd (.*)$"# => | op: Operand | Snd(op),
    SET = r#"set (.*) (.*)$"# => | op1: Operand, op2: Operand | Set(op1, op2),
    ADD = r#"add (.*) (.*)$"# => | op1: Operand, op2: Operand | Add(op1, op2),
    MUL = r#"mul (.*) (.*)$"# => | op1: Operand, op2: Operand | Mul(op1, op2),
    MOD = r#"mod (.*) (.*)$"# => | op1: Operand, op2: Operand | Mod(op1, op2),
    RCV = r#"rcv (.*)$"# => | op: Operand | Rcv(op),
    JGZ = r#"jgz (.*) (.*)$"# => | op1: Operand, op2: Operand | Jgz(op1, op2)
});

fn solve1(input: &str) -> isize {
    let code = input.lines()
                    .map(parse_insn)
                    .collect::<Vec<_>>();
    let mut regs: Vec<isize> = vec![0isize; 26];
    let mut pc = 0isize;
    let mut freq = 0;

    fn get_val(regs: &[isize], op: Operand) -> isize{
        match op {
            Reg(r) => regs[r],
            Val(v) => v,
        }
    };

    loop {
        let mut new_pc = pc + 1;
        assert!(pc >= 0);
        match code[pc as usize] {
            Snd(op) => { freq = get_val(&regs, op) },
            Set(Reg(r), op) => { regs[r] = get_val(&regs, op) },
            Add(Reg(r), op) => { regs[r] += get_val(&regs, op) },
            Mul(Reg(r), op) => { regs[r] *= get_val(&regs, op) },
            Mod(Reg(r), op) => { regs[r] %= get_val(&regs, op) },
            Rcv(op) => { return freq },
            Jgz(op1, op2) => { if get_val(&regs, op1) > 0 { new_pc = pc + get_val(&regs, op2); }; },
            _ => panic!(),
        }
        pc = new_pc;
    }
}

fn main() {
    let input = aoc2017::get_input(18).unwrap();

    assert_eq!(solve1(r#"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2"#), 4);

    println!("Answer 1: {:?}", solve1(&input));
}
