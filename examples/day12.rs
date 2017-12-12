#![feature(conservative_impl_trait)]
#[macro_use] extern crate aoc2017;
#[macro_use] extern crate lazy_static;

use std::collections::HashSet;

struct Connections {
    name: usize,
    links: HashSet<usize>,
}

regex_parser!(parse_pipes: Connections {
    LINE = r#"^(\d+) <-> ([\d, ]+)$"# =>
        | name: usize, pipes: String |
            Connections {
                name: name,
                links: pipes.split(',')
                            .map(str::trim)
                            .map(str::parse)
                            .map(Result::unwrap)
                            .collect(),
            }
});


fn solve(input: &str) -> usize {
    let mut programs: Vec<HashSet<usize>> = Vec::new();;

    for conn in input.lines()
                     .map(parse_pipes)
    {
        assert_eq!(conn.name, programs.len());
        programs.push(conn.links);
    }

    let mut group0 = HashSet::new();
    let mut to_view = vec![0];

    while let Some(item) = to_view.pop() {
        group0.insert(item);
        for connected in programs[item].iter() {
            if !group0.contains(connected) {
                to_view.push(*connected);
            }
        }
    }
    group0.len()
}

fn main() {
    let input = aoc2017::get_input(12).unwrap();

    assert_eq!(solve(r#"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"#), 6);

    println!("Answer: {:?}", solve(&input));
}