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

/* Returns Ok(total_weight) if balanced, or Err(new_weight)
 * if it's found the wrong one.
 */
fn find_wrong_weight(nodes: &HashMap<&str, &TowerInfo>,
                     root: &str) -> Result<usize, usize>
{
    let tinfo = nodes.get(root).unwrap();
    if tinfo.children.len() == 0 {
        return Ok(tinfo.weight);
    }
    let mut child_weights: Vec<usize> = Vec::new();
    for child in &tinfo.children {
        let child_weight = find_wrong_weight(nodes, child.as_str())?;
        child_weights.push(child_weight);
    }
    let child_sum: usize = child_weights.iter().sum();
    if child_sum == (child_weights.len() * child_weights[0]) {
        Ok(tinfo.weight + child_sum)
    } else {
        /* One of them is different! */
        let num_children = child_weights.len();
        assert!(num_children >= 3);
        let (right_weight, wrong_weight) = {
            let mut sorted_weights = child_weights.clone();
            sorted_weights.sort();
            if sorted_weights[0] == sorted_weights[1] {
                (sorted_weights[1], sorted_weights[num_children-1])
            } else {
                (sorted_weights[1], sorted_weights[0])
            }
        };
        for i in 0..num_children {
            if child_weights[i] == wrong_weight {
                /* This is it! */
                let node = nodes.get(tinfo.children[i].as_str()).unwrap();
                return Err(node.weight + right_weight - wrong_weight);
            }
        }
        panic!();
    }
}

fn solve2(input: &str) -> usize {
    let data = input.lines()
                    .map(parse_tinfo)
                    .collect::<Vec<_>>();

    let mut roots: HashSet<&str> = HashSet::new();
    let mut children: HashSet<&str> = HashSet::new();
    let mut nodes: HashMap<&str, &TowerInfo> = HashMap::new();

    for tinfo in &data {
        nodes.insert(&tinfo.name.as_str(), tinfo);
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
    let root: String = roots.into_iter().next().unwrap().into();
    find_wrong_weight(&nodes, &root).unwrap_err()
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

    assert_eq!(solve2(r#"pbga (66)
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
    cntj (57)"#), 60);

    println!("Answer 2: {}", solve2(&input));
}