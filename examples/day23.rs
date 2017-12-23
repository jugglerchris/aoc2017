#![feature(conservative_impl_trait)]
extern crate aoc2017;

use aoc2017::cpu::*;

fn solve1(input: &str) -> usize {
    let code = input.lines()
                    .map(parse_insn)
                    .collect::<Vec<_>>();
    let mut regs: Vec<isize> = vec![0isize; 26];
    let mut pc = 0isize;
    let mut multiplies = 0;

    fn get_val(regs: &[isize], op: Operand) -> isize{
        match op {
            Reg(r) => regs[r],
            Val(v) => v,
        }
    };

    loop {
        let mut new_pc = pc + 1;
        if pc < 0 || (pc as usize) >= code.len() {
            break;
        }
        match code[pc as usize] {
            Set(Reg(r), op) => { regs[r] = get_val(&regs, op) },
            Sub(Reg(r), op) => { regs[r] -= get_val(&regs, op) },
            Mul(Reg(r), op) => { multiplies +=1 ; regs[r] *= get_val(&regs, op) },
            Jnz(op1, op2) => { if get_val(&regs, op1) != 0 { new_pc = pc + get_val(&regs, op2); }; },
            _ => panic!(),
        }
        pc = new_pc;
    }
    multiplies
}

fn main() {
    let input = aoc2017::get_input(23).unwrap();

    println!("Answer 1: {:?}", solve1(&input));
}
