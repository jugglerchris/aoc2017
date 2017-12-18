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

struct Thread<'a> {
    regs: Vec<isize>,
    pc: isize,
    input: Option<isize>,
    code: &'a [Insn],
}

#[derive(Copy,Clone,Debug)]
enum IO {
    Read,
    Send(isize),
}
use IO::*;

impl<'a> Thread<'a> {
    pub fn new(id: isize, code: &'a [Insn]) -> Thread {
        let mut regs = vec![0isize; 26];
        regs[(b'p' - b'a') as usize] = id;
        Thread {
            code: code,
            regs: regs,
            pc: 0,
            input: None
        }
    }

    fn get_val(&self, op: Operand) -> isize {
        match op {
            Reg(r) => self.regs[r],
            Val(v) => v,
        }
    }

    pub fn send(&mut self, val: isize) {
        assert!(self.input.is_none());
        self.input = Some(val);
    }
    pub fn run_to_io(&mut self) -> IO {
        loop {
            let mut new_pc = self.pc + 1;
            assert!(self.pc >= 0);
            match self.code[self.pc as usize] {
                Snd(op) => {
                    self.pc = new_pc;
                    return Send(self.get_val(op));
                },
                Set(Reg(r), op) => { self.regs[r] = self.get_val(op) },
                Add(Reg(r), op) => { self.regs[r] += self.get_val(op) },
                Mul(Reg(r), op) => { self.regs[r] *= self.get_val(op) },
                Mod(Reg(r), op) => { self.regs[r] %= self.get_val(op) },
                Rcv(Reg(r)) => {
                    match self.input.take() {
                        None => {
                            // Don't adjust PC - we want to rerun.
                            return Read;
                        },
                        Some(val) => { self.regs[r] = val; },
                    }
                },
                Jgz(op1, op2) => {
                    if self.get_val(op1) > 0 {
                        new_pc = self.pc + self.get_val(op2);
                    };
                },
                _ => panic!(),
            }
            self.pc = new_pc;
        }
    }
}

fn solve2(input: &str) -> isize {
    let code = input.lines()
                    .map(parse_insn)
                    .collect::<Vec<_>>();

    let mut t0 = Thread::new(0, &code);
    let mut t1 = Thread::new(1, &code);
    let mut sends_from_1 = 0;
    loop {
        let mut t0_blocked = false;
        let mut t1_blocked = false;
        match t0.run_to_io() {
            Read => { t0_blocked = true; },
            Send(v) => t1.send(v),
        }
        match t1.run_to_io() {
            Read => { t1_blocked = true; },
            Send(v) => {
                t0.send(v);
                sends_from_1 += 1;
            },
        }
        if t0_blocked && t1_blocked {
            return sends_from_1;
        }
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

    assert_eq!(solve2(r#"snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d"#), 3);

    println!("Answer 2: {:?}", solve2(&input));
}
