#![feature(conservative_impl_trait)]
#[macro_use] extern crate aoc2017;
#[macro_use] extern crate lazy_static;

use std::collections::*;

#[derive(Debug,Clone, Eq, PartialEq)]
struct TowerInfo {
    name: String,
    weight: usize,
    children: Vec<String>,
}

regex_parser!(parse_tinfo: TowerInfo {
    TOWER = r#"^\s*(\w+) \((\d+)\) -> ([\w, ]*)$"# =>
        | name: String, weight: usize, childstring: String|
            TowerInfo {
                name: name,
                weight: weight,
                children: childstring.split(',')
                                     .map(|s| s.trim())
                                     .map(String::from)
                                     .collect::<Vec<String>>()
            },
    TOP_TOWER = r#"^\s*(\w+) \((\d+)\)$"# => | name: String, weight: usize| TowerInfo {
        name: name, weight: weight, children: Vec::new()
    }
});

#[test]
fn test_parse() {
    assert_eq!(parse_tinfo("blah (123)"), TowerInfo {
        name: "blah".into(),
        weight: 123,
        children: Vec::new()
    });
    assert_eq!(parse_tinfo("blah (123) -> foo, bar, baz"), TowerInfo {
        name: "blah".into(),
        weight: 123,
        children: vec!["foo".into(), "bar".into(), "baz".into()]
    });
}

/*
#[derive(Debug,Clone)]
struct Tower {
    name: String,
    weight: usize,
    parents: usize,
    children: Vec<Rc<Tower>>,
}
*/

fn solve1(input: &str) -> String {
    let data = input.lines()
                    .map(parse_tinfo)
                    .collect::<Vec<_>>();

    let mut roots: HashSet<&str> = HashSet::new();
    let mut children: HashSet<&str> = HashSet::new();

    for tinfo in &data {
        let is_known_child = children.contains(&tinfo.name.as_str());
        if !is_known_child {
            roots.insert(&tinfo.name.as_str());
        }
        for child in &tinfo.children {
            let was_root = roots.contains(&child.as_str());
            if was_root {
                roots.remove(&child.as_str());
            }
            children.insert(&child);
        }
    }
    assert_eq!(roots.len(), 1);
    roots.into_iter().next().unwrap().into()
}

fn main() {
    let input = aoc2017::get_input(7).unwrap();

    assert_eq!(solve1(r#"pbga (66)
    xhth (57)
    ebii (61)
    havc (66)
    ktlj (57)
    fwft (72) -> ktlj, cntj, xhth
    qoyq (66)
    padx (45) -> pbga, havc, qoyq
    tknk (41) -> ugml, padx, fwft
    jptl (61)
    ugml (68) -> gyxo, ebii, jptl
    gyxo (61)
    cntj (57)"#), "tknk");

    println!("Answer 1: {}", solve1(&input));
}