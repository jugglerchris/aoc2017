#![feature(conservative_impl_trait)]
#[macro_use] extern crate aoc2017;

fn solve(input: &str) -> (usize, usize) {
    let mut cs = input.chars();
    let mut depth = 0;
    let mut score = 0;
    let mut garbage = 0;
    while let Some(c) = cs.next() {
        match c {
            '{' => { depth += 1; },
            '<' => {
                loop {
                    match cs.next().unwrap() {
                        '>' => { break; },
                        '!' => { cs.next().unwrap(); },
                        _ => { garbage += 1; },
                    }
                }
            },
            '}' => {
                score += depth;
                depth -= 1;
            },
            _ => {},
        }
    }
    (score, garbage)
}

fn main() {
    let input = aoc2017::get_input(9).unwrap();

    assert_eq!(solve("{}"), (1, 0));
    assert_eq!(solve("{{{}}}"), (6, 0));
    assert_eq!(solve("{{},{}}"), (5, 0));
    assert_eq!(solve("{{{},{},{{}}}}"), (16, 0));
    assert_eq!(solve("{<a>,<a>,<a>,<a>}"), (1, 4));
    assert_eq!(solve("{{<ab>},{<ab>},{<ab>},{<ab>}}"), (9, 8));
    assert_eq!(solve("{{<!!>},{<!!>},{<!!>},{<!!>}}"), (9, 0));
    assert_eq!(solve("{{<a!>},{<a!>},{<a!>},{<ab>}}"), (3, 17));

    assert_eq!(solve("<>"), (0, 0));
    assert_eq!(solve("<random characters>"), (0, 17));
    assert_eq!(solve("<<<<>"), (0, 3));
    assert_eq!(solve("<{!>}>"), (0, 2));
    assert_eq!(solve("<!!>"), (0, 0));
    assert_eq!(solve("<!!!>>"), (0, 0));
    assert_eq!(solve("<{o\"i!a,<{i<a>"), (0, 10));

    println!("Answer: {:?}", solve(&input));
}
