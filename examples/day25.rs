#![feature(conservative_impl_trait)]
extern crate aoc2017;

type Value = usize;

#[derive(Copy,Clone)]
enum Move {
    Left,
    Right,
}
use Move::*;

type StateNum = usize;

#[derive(Copy,Clone)]
struct Action {
    value_to_write: Value,
    move_: Move,
    state: StateNum,
}

#[derive(Copy,Clone)]
struct State {
    actions: [Action;2],
}

struct Machine {
    states: Vec<State>,
    pos: isize,
    state: StateNum,
    steps_to_run: usize,
    tape_pos: Vec<Value>,
    tape_neg: Vec<Value>,
}

impl Machine {
    fn current_val(&mut self) -> Value {
        if self.pos >= 0 {
            let upos = self.pos as usize;
            if self.tape_pos.len() <= upos {
                self.tape_pos.push(0);
            }
            self.tape_pos[upos]
        } else {
            let upos = (-1-self.pos) as usize;
            if self.tape_neg.len() <= upos {
                self.tape_neg.push(0);
            }
            self.tape_neg[upos]
        }
    }

    fn write_val(&mut self, val: Value) {
        if self.pos >= 0 {
            let upos = self.pos as usize;
            if self.tape_pos.len() <= upos {
                self.tape_pos.push(0);
            }
            self.tape_pos[upos] = val;
        } else {
            let upos = (-1-self.pos) as usize;
            if self.tape_neg.len() <= upos {
                self.tape_neg.push(0);
            }
            self.tape_neg[upos] = val;
        }
    }

    fn state(&self) -> &State {
        &self.states[self.state as usize]
    }

    fn step(&mut self) {
        let val = self.current_val();
        let action = self.state().actions[val as usize];
        self.write_val(action.value_to_write);
        self.pos += match action.move_ {
            Left => -1,
            Right => 1,
        };
        self.state = action.state;
    }
    fn checksum(&self) -> usize {
        self.tape_neg.iter().sum::<usize>() + self.tape_pos.iter().sum::<usize>()
    }
}

fn parse_turing(input: &str) -> Machine {
    let mut states = Vec::new();
    let mut lines = input.lines();
    let starting_state = {
        let line = lines.next().unwrap();
        let word = line.split_whitespace().nth(3).unwrap();
        (word.chars().next().unwrap() as u8 - b'A') as StateNum
    };
    let steps_to_run: usize = {
        let line = lines.next().unwrap();
        let word = line.split_whitespace().nth(5).unwrap();
        word.parse().unwrap()
    };

    loop {
        let state_lines = lines.by_ref().take(10).collect::<Vec<_>>();
        if state_lines.len() < 10 {
            break;
        }
        let mut state_lines = state_lines.into_iter();
        // Skip blank line
        state_lines.next().unwrap();
        let last_words = state_lines
                           .map(|l| l.split_whitespace().last().unwrap())
                           .collect::<Vec<&str>>();
        let state_name = last_words[0].chars().next().unwrap();
        assert_eq!(states.len(), (state_name as u8 - b'A') as usize);

        let mut actions = Vec::new();
        for i in 0..2 {
            // "If the current value is..."
            let lw = &last_words[i*4 + 1..i*4+5];
            match lw[0] {
                "0:" => assert_eq!(i, 0),
                "1:" => assert_eq!(i, 1),
                _ => (),
            }
            let write_val = match lw[1] {
                "0." => 0,
                "1." => 1,
                _ => panic!(),
            };
            let move_ = match lw[2] {
                "left." => Left,
                "right." => Right,
                _ => panic!(),
            };
            let next_state = (lw[3].chars().next().unwrap() as u8 - b'A') as usize;
            actions.push(Action {
                value_to_write: write_val,
                move_: move_,
                state: next_state,
            });
        }
        states.push(State {
            actions: [actions[0], actions[1]],
        });
    }
    Machine {
        states: states,
        pos: 0,
        state: starting_state,
        steps_to_run: steps_to_run,
        tape_pos: vec![0],
        tape_neg: Vec::new(),
    }
}

fn solve(input: &str) -> usize {
    let mut machine = parse_turing(input);
    for _ in 0..machine.steps_to_run {
        machine.step();
    }
    machine.checksum()
}

fn main() {
    let input = aoc2017::get_input(25).unwrap();

    assert_eq!(solve(r#"Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
"#), 3);
    println!("Answer 1: {:?}", solve(&input));
}
